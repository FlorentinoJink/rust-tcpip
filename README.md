# rust-tcpip
尝试用rust写一个TcpIp协议

## 项目简介

这是一个学习性质的 TCP/IP 协议栈实现，使用 Rust 从零开始构建网络协议栈的各个层次。通过这个项目可以深入理解网络协议的工作原理。

## Roadmap

### ✅ Phase 1: 项目基础设施
- [x] 搭建项目结构和基础设施

### ✅ Phase 2: 数据链路层
- [x] 实现网络接口层（TAP 设备）
- [x] 实现以太网帧解析和构造
- [x] 实现 ARP 协议和缓存机制

### ✅ Phase 3: 网络层
- [x] 实现 IP 数据包解析和构造
- [x] 实现 IP 校验和和 TTL 处理
- [x] 实现 ICMP 协议（ping 响应）

### 🚧 Phase 4: 传输层 - UDP
- [ ] 实现 UDP 数据报解析和构造
- [ ] 实现 UDP 端口管理和数据分发
- [ ] 实现 UDP echo 示例

### 📋 Phase 5: 传输层 - TCP 基础
- [ ] 实现 TCP 段解析和构造
- [ ] 实现 TCP 连接建立（三次握手）
- [ ] 实现 TCP 连接关闭（四次挥手）

### 📋 Phase 6: TCP 数据传输
- [ ] 实现 TCP 数据发送和接收
- [ ] 实现 TCP 粘包处理和字节流接口
- [ ] 实现 TCP 重传机制
- [ ] 实现 MSS 协商和 Nagle 算法

### 📋 Phase 7: Socket API
- [ ] 实现 Socket 抽象层
- [ ] 实现 TCP Socket 操作
- [ ] 实现 UDP Socket 操作

### 📋 Phase 8: 集成和工具
- [ ] 集成所有协议层
- [ ] 实现 ping 工具
- [ ] 实现 traceroute 工具
- [ ] 实现 UDP/TCP echo 示例程序

## 技术栈

- **语言**: Rust
- **网络设备**: tun-tap
- **日志**: tracing + tracing-subscriber
- **错误处理**: thiserror + anyhow

## 学习资源

- [RFC 791 - Internet Protocol](https://tools.ietf.org/html/rfc791)
- [RFC 792 - Internet Control Message Protocol](https://tools.ietf.org/html/rfc792)
- [RFC 793 - Transmission Control Protocol](https://tools.ietf.org/html/rfc793)
- [RFC 768 - User Datagram Protocol](https://tools.ietf.org/html/rfc768)
- [RFC 826 - Ethernet Address Resolution Protocol](https://tools.ietf.org/html/rfc826)
