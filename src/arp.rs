//! ARP 协议实现
//!
//! ARP（Address Resolution Protocol）用于将 IP 地址解析为 MAC 地址
use crate::error::{Result, StackError};
use std::net::Ipv4Addr;

const ARP_PACKET_LEN: usize = 28;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArpOperation {
    Request,
    Reply,
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
    pub fn parse(data: &[u8]) -> Result<Self> {
        if data.len() < ARP_PACKET_LEN {
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
        let sender_ip = std::net::Ipv4Addr::new(data[14], data[15], data[16], data[17]);

        let mut target_mac = [0u8; 6];
        target_mac.copy_from_slice(&data[18..24]);

        let target_ip = std::net::Ipv4Addr::new(data[24], data[25], data[26], data[27]);

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

}
