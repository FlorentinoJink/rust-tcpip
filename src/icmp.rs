//! ICMP 协议实现
//! 
//! ICMP（Internet Control Message Protocol）用于网络诊断和错误报告

/// ICMP 数据包结构
#[derive(Debug)]
pub struct IcmpPacket {
    pub icmp_type: u8,        // ICMP 类型
    pub code: u8,             // 代码
    pub checksum: u16,        // 校验和
    pub identifier: u16,      // 标识符（用于 ping）
    pub sequence: u16,        // 序列号（用于 ping）
    pub payload: Vec<u8>,     // 数据负载
}

/// ICMP 消息类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IcmpType {
    EchoReply = 0,              // Echo 响应（ping 回复）
    DestinationUnreachable = 3, // 目标不可达
    TimeExceeded = 11,          // 超时（用于 traceroute）
    EchoRequest = 8,            // Echo 请求（ping）
}
