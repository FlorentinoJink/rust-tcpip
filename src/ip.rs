//! IP 层实现
//!
//! IPv4 协议负责数据包的路由和转发

use std::net::Ipv4Addr;

use crate::error::{Result, StackError};

const IP_PACKET_LEN: usize = 20;

/// IPv4 数据包结构
#[derive(Debug)]
pub struct Ipv4Packet {
    pub version: u8,          // 版本号（IPv4 是 4）
    pub ihl: u8,              // 头部长度（Internet Header Length）
    pub tos: u8,              // 服务类型（Type of Service）
    pub total_length: u16,    // 总长度（包括头部和数据）
    pub identification: u16,  // 标识符（用于分片重组）
    pub flags: u8,            // 标志位
    pub fragment_offset: u16, // 片偏移
    pub ttl: u8,              // 生存时间（Time To Live）
    pub protocol: u8,         // 上层协议（6=TCP, 17=UDP, 1=ICMP）
    pub checksum: u16,        // 头部校验和
    pub src_addr: Ipv4Addr,   // 源 IP 地址
    pub dst_addr: Ipv4Addr,   // 目标 IP 地址
    pub payload: Vec<u8>,     // 数据负载
}

impl Ipv4Packet {
    pub fn parse(data: &[u8]) -> Result<Self> {
        if data.len() < IP_PACKET_LEN {
            return Err(StackError::InvalidPacket(String::from(
                "Ip packet too short",
            )));
        }
        // 字节 0： 版本号(高四位) + ihs(低四位)
        let version = data[0] >> 4;
        let ihl = data[0] & 0x0F;

        // 字节 1： tos
        let tos = data[1];

        // 字节 2-3： 总长度
        let total_length = u16::from_be_bytes([data[2], data[3]]);

        // 字节 4-5： 标识符
        let identification = u16::from_be_bytes([data[4], data[5]]);
        // 字节 6-7： 标志位flag + 片偏移
        let flags_and_offset = u16::from_be_bytes([data[6], data[7]]);

        // 标志位占用3字节
        let flags = (flags_and_offset >> 13) as u8; // 右移动取出高位
        // 片偏移占用 13个字节
        let fragment_offset = flags_and_offset & 0x1FFF; // 低位 &上13个 1

        // 字节 8： TTL
        let ttl = data[8];

        // 字节 9： 协议
        let protocol = data[9];

        // 字节 10-11： 校验和
        let checksum = u16::from_be_bytes([data[10], data[11]]);

        // 字节12-15： 源地址
        let src_addr = Ipv4Addr::new(data[12], data[13], data[14], data[15]);

        // 字节16-19： 目标地址
        let dst_addr = Ipv4Addr::new(data[16], data[17], data[18], data[19]);

        // 计算头部长度（IHL * 4 字节）
        let _header_len = (ihl as usize) * 4;

        let payload = data[22..].to_vec();
        Ok(Self {
            version,
            ihl,
            tos,
            total_length,
            identification,
            flags,
            fragment_offset,
            ttl,
            protocol,
            checksum,
            src_addr,
            dst_addr,
            payload,
        })
    }
}
