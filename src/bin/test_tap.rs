use rust_tcpip::arp::{ArpModule, ArpPacket};
use rust_tcpip::device::*;
use rust_tcpip::ethernet::{EtherType, EthernetFrame, FramePayload};
use std::net::Ipv4Addr;
use tracing::info;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    rust_tcpip::init_tracing();

    let mut device = TapDevice::new("tap0")?;
    let our_ip = Ipv4Addr::new(192, 168, 10, 1);
    let our_mac = [66, 66, 66, 66, 66, 66];
    let netmask = Ipv4Addr::new(255, 255, 255, 0);
    device.set_ip(our_ip, netmask)?;

    info!("TAP device created and configured!");
    let mut arp_module = ArpModule::new(our_ip, our_mac);
    let mut buf = [0u8; 1500];

    loop {
        let read_size = device.recv(&mut buf)?;
        let ethernet_frame = EthernetFrame::parse(&buf[..read_size])?;

        match ethernet_frame.classify_payload() {
            FramePayload::Arp(data) => {
                let arp = ArpPacket::parse(&data)?;
                if let Some(arp_response) = arp_module.handle_packet(&arp) {
                    let response_frame = EthernetFrame::build(
                        arp.sender_mac,
                        our_mac,
                        EtherType::to_u16(EtherType::ARP),
                        arp_response,
                    );
                    device.send(&response_frame.to_bytes())?;
                    info!("Send ARP reply");
                }
            }
            FramePayload::Ipv4(_data) => {}
            _ => {
                info!("Unknown packet");
            }
        }
    }
}
