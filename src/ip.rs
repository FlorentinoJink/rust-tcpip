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
        let header_len = (ihl as usize) * 4;

        // payload 从头部结束后开始
        let payload = data[header_len..].to_vec();
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

    pub fn build(
        src_addr: Ipv4Addr,
        dst_addr: Ipv4Addr,
        protocol: u8,
        ttl: u8,
        payload: Vec<u8>,
    ) -> Self {
        let total_length = 20 + payload.len() as u16; // 头部 20 字节 + 负载

        Self {
            version: 4,
            ihl: 5, // 5 * 4 = 20 字节（无选项）
            tos: 0,
            total_length,
            identification: 0, // 可以用随机数或计数器
            flags: 0,
            fragment_offset: 0,
            ttl,
            protocol,
            checksum: 0, // 稍后计算
            src_addr,
            dst_addr,
            payload,
        }
    }

    /// 序列化为字节数组
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(20 + self.payload.len());

        // 第 1 行：版本(4bit) + IHL(4bit) + TOS(8bit) + 总长度(16bit)
        bytes.push((self.version << 4) | self.ihl);
        bytes.push(self.tos);
        bytes.extend_from_slice(&self.total_length.to_be_bytes());

        // 第 2 行：标识(16bit) + 标志(3bit) + 片偏移(13bit)
        bytes.extend_from_slice(&self.identification.to_be_bytes());
        let flags_offset = ((self.flags as u16) << 13) | self.fragment_offset;
        bytes.extend_from_slice(&flags_offset.to_be_bytes());

        // 第 3 行：TTL(8bit) + 协议(8bit) + 校验和(16bit)
        bytes.push(self.ttl);
        bytes.push(self.protocol);
        bytes.extend_from_slice(&[0, 0]); // 校验和占位

        // 第 4-5 行：源 IP 和目标 IP
        bytes.extend_from_slice(&self.src_addr.octets());
        bytes.extend_from_slice(&self.dst_addr.octets());

        // 计算校验和（只计算头部 20 字节）
        let checksum = Self::calculate_ip_checksum(&bytes[..20]);
        bytes[10..12].copy_from_slice(&checksum.to_be_bytes());

        // 添加负载
        bytes.extend_from_slice(&self.payload);

        bytes
    }

    /// 计算 IP 校验和
    fn calculate_ip_checksum(header: &[u8]) -> u16 {
        let mut sum: u32 = 0;

        for chunk in header.chunks(2) {
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
