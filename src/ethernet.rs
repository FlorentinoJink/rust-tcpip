//! 以太网层实现
//!
//! 负责以太网帧的解析和构造

use crate::error::{Result, StackError};

const ETHER_MIN_BYTES: usize = 14;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum EtherType {
    IPv4 = 0x0800,
    ARP = 0x0806,
    IPv6 = 0x86DD,
}

impl EtherType {
    pub fn from_u16(value: u16) -> Option<Self> {
        match value {
            0x0800 => Some(EtherType::IPv4),
            0x0806 => Some(EtherType::ARP),
            0x86DD => Some(EtherType::IPv6),
            _ => None,
        }
    }
    pub fn to_u16(ether_type: EtherType) -> u16 {
        match ether_type {
            EtherType::ARP => 0x0806,
            EtherType::IPv4 => 0x0800,
            EtherType::IPv6 => 0x86DD,
        }
    }
}

#[derive(Debug)]
pub enum FramePayload {
    Arp(Vec<u8>),
    Ipv4(Vec<u8>),
    Ipv6(Vec<u8>),
    Unknown,
}

/// 以太网帧结构
#[derive(Debug)]
pub struct EthernetFrame {
    pub dst_mac: [u8; 6], // 目标 MAC 地址
    pub src_mac: [u8; 6], // 源 MAC 地址
    pub ether_type: u16,  // 以太网类型（0x0800=IPv4, 0x0806=ARP）
    pub payload: Vec<u8>, // 数据负载
}

impl EthernetFrame {
    pub fn get_ether_type(&self) -> Option<EtherType> {
        EtherType::from_u16(self.ether_type)
    }
    pub fn parse(data: &[u8]) -> Result<Self> {
        // 以太网最小数据帧: 6 + 6 + 2 = 14 bytes
        if data.len() < ETHER_MIN_BYTES {
            return Err(StackError::InvalidPacket(String::from(
                "Ethernet frame too short",
            )));
        }

        // 提取 dst_mac
        let mut dst_mac = [0u8; 6];
        dst_mac.copy_from_slice(&data[0..6]);
        // 提取 src_mac
        let mut src_mac = [0u8; 6];
        src_mac.copy_from_slice(&data[6..12]);
        // 以太网类型
        let ether_type = u16::from_be_bytes([data[12], data[13]]);
        // 数据负载
        let payload = data[14..].to_vec();

        Ok(Self {
            src_mac,
            dst_mac,
            ether_type,
            payload,
        })
    }

    pub fn classify_payload(&self) -> FramePayload {
        match self.get_ether_type() {
            Some(EtherType::ARP) => FramePayload::Arp(self.payload.clone()),
            Some(EtherType::IPv4) => FramePayload::Ipv4(self.payload.clone()),
            Some(EtherType::IPv6) => FramePayload::Ipv6(self.payload.clone()),
            None => FramePayload::Unknown,
        }
    }

    pub fn build(dst_mac: [u8; 6], src_mac: [u8; 6], ether_type: u16, payload: Vec<u8>) -> Self {
        Self {
            dst_mac,
            src_mac,
            ether_type,
            payload,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.payload.len());
        bytes.extend_from_slice(&self.dst_mac);
        bytes.extend_from_slice(&self.src_mac);
        bytes.extend_from_slice(&self.ether_type.to_be_bytes());
        bytes.extend_from_slice(&self.payload);
        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ethernet_frame() {
        let mut data = vec![
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // 目标 MAC
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, // 源 MAC
            0x08, 0x00, // EtherType (IPv4)
        ];
        data.extend(b"payload");

        let frame = EthernetFrame::parse(&data).unwrap();

        assert_eq!(frame.ether_type, 0x0800);
        assert_eq!(frame.payload, b"payload");
    }
}
