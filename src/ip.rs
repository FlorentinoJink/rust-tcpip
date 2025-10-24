//! IP 层实现
//! 
//! IPv4 协议负责数据包的路由和转发

use std::net::Ipv4Addr;

/// IPv4 数据包结构
#[derive(Debug)]
pub struct Ipv4Packet {
    pub version: u8,             // 版本号（IPv4 是 4）
    pub ihl: u8,                 // 头部长度（Internet Header Length）
    pub tos: u8,                 // 服务类型（Type of Service）
    pub total_length: u16,       // 总长度（包括头部和数据）
    pub identification: u16,     // 标识符（用于分片重组）
    pub flags: u8,               // 标志位
    pub fragment_offset: u16,    // 片偏移
    pub ttl: u8,                 // 生存时间（Time To Live）
    pub protocol: u8,            // 上层协议（6=TCP, 17=UDP, 1=ICMP）
    pub checksum: u16,           // 头部校验和
    pub src_addr: Ipv4Addr,      // 源 IP 地址
    pub dst_addr: Ipv4Addr,      // 目标 IP 地址
    pub payload: Vec<u8>,        // 数据负载
}