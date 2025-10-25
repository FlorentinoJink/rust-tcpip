//! 以太网层实现
//!
//! 负责以太网帧的解析和构造

use crate::error::{Result, StackError};

const ETHER_MIN_BYTES: usize = 14;

/// 以太网帧结构
#[derive(Debug)]
pub struct EthernetFrame {
    pub dst_mac: [u8; 6], // 目标 MAC 地址
    pub src_mac: [u8; 6], // 源 MAC 地址
    pub ether_type: u16,  // 以太网类型（0x0800=IPv4, 0x0806=ARP）
    pub payload: Vec<u8>, // 数据负载
}

impl EthernetFrame {
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
