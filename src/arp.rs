//! ARP 协议实现
//! 
//! ARP（Address Resolution Protocol）用于将 IP 地址解析为 MAC 地址

use std::net::Ipv4Addr;

/// ARP 数据包结构
#[derive(Debug)]
pub struct ArpPacket {
    pub operation: u16,          // 操作类型：1=请求，2=响应
    pub sender_mac: [u8; 6],     // 发送方 MAC 地址
    pub sender_ip: Ipv4Addr,     // 发送方 IP 地址
    pub target_mac: [u8; 6],     // 目标 MAC 地址
    pub target_ip: Ipv4Addr,     // 目标 IP 地址
}
