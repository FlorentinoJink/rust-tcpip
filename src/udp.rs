//! UDP 协议实现
//!
//! UDP（User Datagram Protocol）是无连接的传输层协议

use std::net::Ipv4Addr;

/// UDP 数据报结构
use crate::error::{Result, StackError};

const UDP_DATA_GRAM_MIN_SIZE: usize = 8;

#[derive(Debug)]
pub struct UdpDatagram {
    pub src_port: u16,    // 源端口
    pub dst_port: u16,    // 目标端口
    pub length: u16,      // 长度（包括头部和数据）
    pub checksum: u16,    // 校验和
    pub payload: Vec<u8>, // 数据负载
}

impl UdpDatagram {
    pub fn parse(data: &[u8]) -> Result<Self> {
        if data.len() < UDP_DATA_GRAM_MIN_SIZE {
            return Err(StackError::InvalidPacket(String::from(
                "Udp datagram too short",
            )));
        }
        let src_port = u16::from_be_bytes([data[0], data[1]]);
        let dst_port = u16::from_be_bytes([data[2], data[3]]);
        let length = u16::from_be_bytes([data[4], data[5]]);
        let checksum = u16::from_be_bytes([data[6], data[7]]);
        let payload = data[8..].to_vec();
        Ok(Self {
            src_port,
            dst_port,
            length,
            checksum,
            payload,
        })
    }

    pub fn build(
        src_port: u16,
        dst_port: u16,
        payload: Vec<u8>,
        src_addr: Ipv4Addr,
        dst_addr: Ipv4Addr,
    ) -> Self {
        let length = (8 + payload.len()) as u16; // UDP 头部 8 字节 + 数据
        let mut datagram = Self {
            src_port,
            dst_port,
            length,
            checksum: 0,
            payload,
        };
        // 计算校验和
        datagram.checksum = Self::calculate_udp_checksum(&datagram, src_addr, dst_addr);
        datagram
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.src_port.to_be_bytes());
        bytes.extend_from_slice(&self.dst_port.to_be_bytes());
        bytes.extend_from_slice(&self.length.to_be_bytes());
        bytes.extend_from_slice(&self.checksum.to_be_bytes());
        bytes.extend_from_slice(&self.payload);
        bytes
    }

    pub fn build_echo(request: &UdpDatagram, src_addr: Ipv4Addr, dst_addr: Ipv4Addr) -> Self {
        let mut echo = Self {
            src_port: request.dst_port,
            dst_port: request.src_port,
            length: request.length,
            checksum: 0,
            payload: request.payload.clone(),
        };
        echo.checksum = Self::calculate_udp_checksum(&echo, src_addr, dst_addr);
        echo
    }

    fn calculate_udp_checksum(
        datagram: &UdpDatagram,
        src_addr: Ipv4Addr,
        dst_addr: Ipv4Addr,
    ) -> u16 {
        let mut data = Vec::new();

        // 1. UDP 伪头部
        data.extend_from_slice(&src_addr.octets());
        data.extend_from_slice(&dst_addr.octets());
        data.push(0);
        data.push(17); // UDP 协议号
        data.extend_from_slice(&datagram.length.to_be_bytes());

        // 2. UDP 头部和数据
        data.extend_from_slice(&datagram.src_port.to_be_bytes());
        data.extend_from_slice(&datagram.dst_port.to_be_bytes());
        data.extend_from_slice(&datagram.length.to_be_bytes());
        data.extend_from_slice(&[0, 0]); // checksum 占位
        data.extend_from_slice(&datagram.payload);

        // 3.计算校验和
        Self::calculate_checksum(&data)
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
