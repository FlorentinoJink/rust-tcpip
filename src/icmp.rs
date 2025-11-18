//! ICMP 协议实现
//!
//! ICMP（Internet Control Message Protocol）用于网络诊断和错误报告

use crate::error::{Result, StackError};

const ICMP_PACKET_MIN_LEN: usize = 8;

/// ICMP 数据包结构
#[derive(Debug)]
pub struct IcmpPacket {
    pub icmp_type: u8,    // ICMP 类型
    pub code: u8,         // 代码
    pub checksum: u16,    // 校验和
    pub identifier: u16,  // 标识符（用于 ping）
    pub sequence: u16,    // 序列号（用于 ping）
    pub payload: Vec<u8>, // 数据负载
}

/// ICMP 消息类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IcmpType {
    EchoReply = 0,              // Echo 响应（ping 回复）
    DestinationUnreachable = 3, // 目标不可达
    TimeExceeded = 11,          // 超时（用于 traceroute）
    EchoRequest = 8,            // Echo 请求（ping）
}

impl IcmpType {
    pub fn to_u8(icmp_type: IcmpType) -> u8 {
        match icmp_type {
            IcmpType::EchoReply => 0,
            IcmpType::DestinationUnreachable => 3,
            IcmpType::TimeExceeded => 11,
            IcmpType::EchoRequest => 8,
        }
    }
    pub fn from_u8(value: u8) -> Option<IcmpType> {
        match value {
            0 => Some(IcmpType::EchoReply),
            3 => Some(IcmpType::DestinationUnreachable),
            11 => Some(IcmpType::TimeExceeded),
            8 => Some(IcmpType::EchoRequest),
            _ => None,
        }
    }
}

impl IcmpPacket {
    pub fn parse(data: &[u8]) -> Result<Self> {
        if data.len() < ICMP_PACKET_MIN_LEN {
            return Err(StackError::InvalidPacket(String::from(
                "Icmp packet too short",
            )));
        };
        let icmp_type = data[0];
        let code = data[1];
        let checksum = u16::from_be_bytes([data[2], data[3]]);
        let identifier = u16::from_be_bytes([data[4], data[5]]);
        let sequence = u16::from_be_bytes([data[6], data[7]]);
        let payload = data[8..].to_vec();
        Ok(Self {
            icmp_type,
            code,
            checksum,
            identifier,
            sequence,
            payload,
        })
    }
    pub fn build_reply(request: &IcmpPacket) -> Self {
        Self {
            icmp_type: IcmpType::to_u8(IcmpType::EchoReply),
            code: 0,
            checksum: 0,
            identifier: request.identifier,
            sequence: request.sequence,
            payload: request.payload.clone(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(8 + self.payload.len());
        bytes.push(self.icmp_type);
        bytes.push(self.code);
        bytes.extend_from_slice(&[0, 0]);
        bytes.extend_from_slice(&self.identifier.to_be_bytes());
        bytes.extend_from_slice(&self.sequence.to_be_bytes());
        bytes.extend_from_slice(&self.payload);
        // 计算校验和
        let checksum = Self::calculate_checksum(&bytes);
        bytes[2..4].copy_from_slice(&checksum.to_be_bytes());

        bytes
    }

    fn calculate_checksum(data: &[u8]) -> u16 {
        let mut sum: u32 = 0;

        for chunk in data.chunks(2) {
            let word = if chunk.len() == 2 {
                u16::from_be_bytes([chunk[0], chunk[1]]) as u32
            } else {
                (chunk[0] as u32) << 8
            };
            sum += word;
        }

        while sum >> 16 != 0 {
            sum = (sum & 0xFFFF) + (sum >> 16);
        }
        !sum as u16
    }
}
