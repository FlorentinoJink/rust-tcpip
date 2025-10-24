//! 以太网层实现
//! 
//! 负责以太网帧的解析和构造

/// 以太网帧结构
#[derive(Debug)]
pub struct EthernetPacket {
    pub s_mac: [u8; 6],      // 源 MAC 地址
    pub d_mac: [u8; 6],      // 目标 MAC 地址
    pub ether_type: u16,     // 以太网类型（0x0800=IPv4, 0x0806=ARP）
    pub payload: Vec<u8>,    // 数据负载
}