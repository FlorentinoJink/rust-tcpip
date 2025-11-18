//! ARP 协议实现
//!
//! ARP（Address Resolution Protocol）用于将 IP 地址解析为 MAC 地址
use tracing::info;

use crate::error::{Result, StackError};
use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::time::{Duration, Instant};

const ARP_PACKET_MIN_LEN: usize = 28;

pub type MacAddr = [u8; 6];

#[derive(Debug)]
pub struct ArpCache {
    entries: HashMap<Ipv4Addr, (MacAddr, Instant)>,
    timeout: Duration,
}

impl ArpCache {
    pub fn new(timeout: Duration) -> Self {
        Self {
            entries: HashMap::new(),
            timeout,
        }
    }

    // 更新缓存
    pub fn insert(&mut self, ip: Ipv4Addr, mac: MacAddr) {
        self.entries.insert(ip, (mac, Instant::now()));
        info!("ARP cache insert: {} -> {:02x?}", ip, mac);
    }

    // 查找mac地址
    pub fn loopup(&mut self, ip: &Ipv4Addr) -> Option<MacAddr> {
        if let Some((mac, timestamp)) = self.entries.get(ip) {
            if timestamp.elapsed() < self.timeout {
                // 缓存命中
                info!("ARP cache hit: {} -> {:02x?}", ip, mac);
                return Some(*mac);
            } else {
                // 缓存过期
                info!("ARP cache expired: {}", ip);
                self.entries.remove(ip);
            }
        }
        None
    }
    // 清理过期缓存
    pub fn clean_up(&mut self) {
        self.entries
            .retain(|_, (_, timestamp)| timestamp.elapsed() < self.timeout);
    }

    // 删除缓存
    pub fn remove(&mut self, ip: &Ipv4Addr) -> Option<MacAddr> {
        self.entries.remove(ip).map(|(mac, _)| mac)
    }
}

#[derive(Debug)]
pub struct ArpModule {
    cache: ArpCache,
    our_ip: Ipv4Addr,
    our_mac: MacAddr,
}

impl ArpModule {
    pub fn new(our_ip: Ipv4Addr, our_mac: MacAddr) -> Self {
        Self {
            cache: ArpCache::new(Duration::from_secs(300)),
            our_ip,
            our_mac,
        }
    }

    pub fn handle_packet(&mut self, arp: &ArpPacket) -> Option<Vec<u8>> {
        self.cache.insert(arp.sender_ip, arp.sender_mac);
        let arp_operation = ArpOperation::from_u16(arp.operation);
        if arp_operation == Some(ArpOperation::Request) && arp.target_ip == self.our_ip {
            let reply = ArpPacket::build_reply(arp, self.our_mac);
            return Some(reply.to_bytes());
        }
        info!("Arp packet is not for us, skip");
        None
    }

    pub fn resolve(&mut self, ip: Ipv4Addr) -> Option<MacAddr> {
        self.cache.loopup(&ip)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ArpOperation {
    Request = 1,
    Reply = 2,
    RArpRequest = 3,
    RArpReply = 4,
}

impl ArpOperation {
    pub fn from_u16(value: u16) -> Option<Self> {
        match value {
            1 => Some(ArpOperation::Request),
            2 => Some(ArpOperation::Reply),
            3 => Some(ArpOperation::RArpRequest),
            4 => Some(ArpOperation::RArpReply),
            _ => None,
        }
    }
    pub fn to_u16(arp_operation: ArpOperation) -> u16 {
        match arp_operation {
            ArpOperation::Request => 1,
            ArpOperation::Reply => 2,
            ArpOperation::RArpRequest => 3,
            ArpOperation::RArpReply => 4,
        }
    }
}

/// ARP 数据包结构
#[derive(Debug)]
pub struct ArpPacket {
    pub hardware_type: u16,
    pub protocol_type: u16,
    pub hardware_len: u8,
    pub procotol_len: u8,
    pub operation: u16,      // 操作类型：1=请求，2=响应
    pub sender_mac: [u8; 6], // 发送方 MAC 地址
    pub sender_ip: Ipv4Addr, // 发送方 IP 地址
    pub target_mac: [u8; 6], // 目标 MAC 地址
    pub target_ip: Ipv4Addr, // 目标 IP 地址
}

impl ArpPacket {
    // 解析Arp请求
    pub fn parse(data: &[u8]) -> Result<Self> {
        if data.len() < ARP_PACKET_MIN_LEN {
            return Err(StackError::InvalidPacket(String::from(
                "ARP packet too short",
            )));
        }
        let hardware_type = u16::from_be_bytes([data[0], data[1]]);
        let protocol_type = u16::from_be_bytes([data[2], data[3]]);
        let hardware_len = data[4];
        let procotol_len = data[5];
        let operation = u16::from_be_bytes([data[6], data[7]]);

        let mut sender_mac = [0u8; 6];
        sender_mac.copy_from_slice(&data[8..14]);
        let sender_ip = Ipv4Addr::new(data[14], data[15], data[16], data[17]);

        let mut target_mac = [0u8; 6];
        target_mac.copy_from_slice(&data[18..24]);

        let target_ip = Ipv4Addr::new(data[24], data[25], data[26], data[27]);

        Ok(Self {
            hardware_type,
            protocol_type,
            hardware_len,
            procotol_len,
            operation,
            sender_mac,
            sender_ip,
            target_mac,
            target_ip,
        })
    }

    // 构建Arp请求， Arp请求的目的是为了得到目标ip
    pub fn build_request(sender_mac: [u8; 6], sender_ip: Ipv4Addr, target_ip: Ipv4Addr) -> Self {
        Self {
            hardware_type: 1,      // Ethernet
            protocol_type: 0x0800, // ipv4
            hardware_len: 6,
            procotol_len: 4,
            operation: 1, // 请求
            sender_mac,
            sender_ip,
            target_mac: [0u8; 6], // 发起ARP请求的时候不知道目标的mac
            target_ip,
        }
    }

    // 构建Arp响应， Arp响应的目的是响应自己的mac给发送方
    pub fn build_reply(request: &ArpPacket, our_mac: [u8; 6]) -> Self {
        Self {
            hardware_type: 1,      // Ethernet
            protocol_type: 0x0800, // ipv4
            hardware_len: 6,
            procotol_len: 4,
            operation: 2,                   // Arp响应
            sender_mac: our_mac,            // 响应给请求者自己的mac
            sender_ip: request.target_ip,   // Arp请求的目标ip就是 Arp回复的发送ip
            target_mac: request.sender_mac, // 响应的目标是发送方
            target_ip: request.sender_ip,   // Arp请求的发送ip就是 Arp回复的目标ip
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(ARP_PACKET_MIN_LEN);
        bytes.extend_from_slice(&self.hardware_type.to_be_bytes());
        bytes.extend_from_slice(&self.protocol_type.to_be_bytes());
        bytes.push(self.hardware_len);
        bytes.push(self.procotol_len);
        bytes.extend_from_slice(&self.operation.to_be_bytes());
        bytes.extend_from_slice(&self.sender_mac);
        bytes.extend_from_slice(&self.sender_ip.octets());
        bytes.extend_from_slice(&self.target_mac);
        bytes.extend_from_slice(&self.target_ip.octets());
        bytes
    }
}
