//! Socket API 抽象层
//!
//! 提供类似操作系统的 socket 接口，供应用程序使用

use std::net::SocketAddrV4;

/// Socket 类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketType {
    Tcp, // TCP socket
    Udp, // UDP socket
}

/// Socket 结构
#[derive(Debug)]
pub struct Socket {
    pub socket_type: SocketType,           // Socket 类型
    pub local_addr: Option<SocketAddrV4>,  // 本地地址（IP + 端口）
    pub remote_addr: Option<SocketAddrV4>, // 远程地址（IP + 端口）
}

/// Socket 管理器
/// 负责管理所有的 socket 连接
#[derive(Debug)]
pub struct SocketManager {
    // 后续会添加 socket 列表等字段
}
