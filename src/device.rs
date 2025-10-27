use crate::error::{Result, StackError};
use std::net::Ipv4Addr;
use std::process::Command;
use tracing::debug;
use tun_tap::{Iface, Mode};

pub trait NetworkDevice {
    fn recv(&mut self, buf: &mut [u8]) -> Result<usize>;
    fn send(&mut self, buf: &[u8]) -> Result<usize>;
}

pub struct NetworkInterface {
    pub device: Box<dyn NetworkDevice>,
    pub ip: Ipv4Addr,
    pub mac: [u8; 6],
    pub netmask: Ipv4Addr,
    pub mtu: usize,
}

impl NetworkInterface {
    pub fn new(
        device: Box<dyn NetworkDevice>,
        ip: Ipv4Addr,
        mac: [u8; 6],
        netmask: Ipv4Addr,
        mtu: usize,
    ) -> Self {
        Self {
            device,
            ip,
            mac,
            netmask,
            mtu,
        }
    }

    pub fn recv_frame(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.device.recv(buf)
    }
    pub fn send_frame(&mut self, buf: &[u8]) -> Result<usize> {
        self.device.send(buf)
    }
}

pub struct TapDevice {
    iface: Iface,
}

impl TapDevice {
    pub fn new(name: &str) -> Result<Self> {
        // new会附带PI头 PacketInfo 改用 without_packet_info
        let iface = Iface::without_packet_info(name, Mode::Tap)?;
        Ok(Self { iface })
    }
    pub fn set_ip(&mut self, ip: Ipv4Addr, netmask: Ipv4Addr) -> Result<()> {
        let iface_name = self.iface.name();

        // 1. 配置ip命令
        // ip addr add 192.168.10.1/24 dev tap0
        let output = Command::new("ip")
            .args(&[
                "addr",
                "add",
                &format!("{}/{}", ip, netmask_to_prefix(netmask)),
                "dev",
                iface_name,
            ])
            .output()?;

        if !output.status.success() {
            return Err(StackError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to set IP address"),
            )));
        }

        // 2. 启动接口
        // ip set dev tap1 up
        Command::new("ip")
            .args(&["link", "set", "dev", iface_name, "up"])
            .output()?;
        Ok(())
    }
}

// 辅助函数：将子网掩码转换为前缀长度
fn netmask_to_prefix(netmask: Ipv4Addr) -> u8 {
    let octets = netmask.octets();
    let mask = u32::from_be_bytes(octets);
    mask.count_ones() as u8
}

impl NetworkDevice for TapDevice {
    fn recv(&mut self, buf: &mut [u8]) -> Result<usize> {
        let bufsize = self.iface.recv(buf)?;
        debug!("Recv {} bytes from TAP device", bufsize);
        Ok(bufsize)
    }
    fn send(&mut self, buf: &[u8]) -> Result<usize> {
        Ok(self.iface.send(buf)?)
    }
}

#[cfg(test)]
mod tests {
    use Ipv4Addr;

    use super::*;
    #[test]
    pub fn test_prefix() {
        let mask_addr = Ipv4Addr::new(255, 255, 255, 0);
        assert_eq!(netmask_to_prefix(mask_addr), 24);
    }
}
