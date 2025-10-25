use rust_tcpip::ethernet::EthernetFrame;
use rust_tcpip::{arp::ArpPacket, device::*};
use tracing::{debug, info};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    rust_tcpip::init_tracing();

    let mut device = TapDevice::new("tap0")?;
    let ip = std::net::Ipv4Addr::new(192, 168, 10, 1);
    let netmask = std::net::Ipv4Addr::new(255, 255, 255, 0);
    device.set_ip(ip, netmask)?;

    info!("TAP device created and configured!");

    let mut buf = [0u8; 1500];

    loop {
        let read_size = device.recv(&mut buf)?;
        let ethernet_frame = EthernetFrame::parse(&buf[..read_size])?;
        debug!(
            "Recv {} bytes. ether_type: 0x{:02x}, payload: {:?}",
            read_size, ethernet_frame.ether_type, ethernet_frame.payload
        );

        // 解析arp包
        if ethernet_frame.ether_type == 0x0806 {
            let arp_packet = ArpPacket::parse(&ethernet_frame.payload);
            info!("ARP: {:?}", arp_packet)
        }
    }
}
