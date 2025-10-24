//! UDP 协议实现
//! 
//! UDP（User Datagram Protocol）是无连接的传输层协议

/// UDP 数据报结构
#[derive(Debug)]
pub struct UdpDatagram {
    pub src_port: u16,      // 源端口
    pub dst_port: u16,      // 目标端口
    pub length: u16,        // 长度（包括头部和数据）
    pub checksum: u16,      // 校验和
    pub payload: Vec<u8>,   // 数据负载
}
