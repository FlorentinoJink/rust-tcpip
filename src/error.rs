//! 错误类型定义
//! 
//! 定义协议栈中可能出现的各种错误

use thiserror::Error;

/// 协议栈错误类型
#[derive(Debug, Error)]
pub enum StackError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),      // IO 错误，读写网络设备失败
    
    #[error("Invalid packet: {0}")] 
    InvalidPacket(String),           // 无效的数据包，格式不正确
    
    #[error("Checksum mismatch")]
    ChecksumMismatch(String),                // 校验和不匹配
    
    #[error("Connection failed")]
    ConnectionFailed(String),                // 连接失败
}

/// Result 类型别名，方便使用
pub type Result<T> = std::result::Result<T, StackError>;
