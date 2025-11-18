use rust_tcpip::arp::{ArpModule, ArpPacket};
use rust_tcpip::device::*;
use rust_tcpip::ethernet::{EtherType, EthernetFrame, FramePayload};
use rust_tcpip::icmp::{IcmpPacket, IcmpType};
use rust_tcpip::ip::Ipv4Packet;
use rust_tcpip::udp::UdpDatagram;
use std::net::Ipv4Addr;
use tracing::info;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    rust_tcpip::init_tracing();

    let mut device = TapDevice::new("tap0")?;
    let our_ip = Ipv4Addr::new(192, 168, 10, 2);
    let tap_ip = Ipv4Addr::new(192, 168, 10, 1);

    let our_mac = [66, 66, 66, 66, 66, 66];
    let netmask = Ipv4Addr::new(255, 255, 255, 0);
    device.set_ip(tap_ip, netmask)?;
    info!("TAP device created and configured!");
    let mut arp_module = ArpModule::new(our_ip, our_mac);
    let mut buf = [0u8; 1500];

    loop {
        let read_size = device.recv(&mut buf)?;
        info!("Received {} bytes", read_size);

        let ethernet_frame = EthernetFrame::parse(&buf[..read_size])?;
        info!("EtherType: {:?}", ethernet_frame.get_ether_type());

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
            FramePayload::Ipv4(data) => {
                let ipv4 = Ipv4Packet::parse(&data)?;
                info!(
                    "Recving ipv4 Packet src_addr: {}",
                    ipv4.src_addr.to_string()
                );
                info!(
                    "Recving ipv4 Packet dst_addr: {}",
                    ipv4.dst_addr.to_string()
                );
                info!("IP protocol: {}", ipv4.protocol);
                // icmp protocol 1
                if ipv4.protocol == 1 {
                    info!("Received ICMP packet, parsing...");
                    let icmp = IcmpPacket::parse(&ipv4.payload)?;
                    info!("ICMP type: {}, code: {}", icmp.icmp_type, icmp.code);
                    if IcmpType::from_u8(icmp.icmp_type) == Some(IcmpType::EchoRequest) {
                        let icmp_reply = IcmpPacket::build_reply(&icmp);
                        let icmp_bytes = icmp_reply.to_bytes();

                        let ip_reply = Ipv4Packet::build(
                            ipv4.dst_addr,
                            ipv4.src_addr,
                            ipv4.protocol,
                            0,
                            icmp_bytes,
                        );
                        let ipv4_bytes = ip_reply.to_bytes();

                        let eth_frame = EthernetFrame::build(
                            ethernet_frame.src_mac,
                            ethernet_frame.dst_mac,
                            EtherType::to_u16(EtherType::IPv4),
                            ipv4_bytes,
                        );
                        device.send(&eth_frame.to_bytes())?;
                        info!("Sending ping reply");
                    }
                }
                // udp protocol 17
                if ipv4.protocol == 17 {
                    info!("Received UDP packet, parsing...");
                    let udp = UdpDatagram::parse(&ipv4.payload)?;
                    info!(
                        "Udp from {}:{} to {}:{}, length: {}",
                        ipv4.src_addr, udp.src_port, ipv4.dst_addr, udp.dst_port, udp.length
                    );
                    info!("Udp payload hex: {:02x?}", udp.payload);
                    let payload_str = String::from_utf8_lossy(&udp.payload);
                    info!("Udp payload str: {}", payload_str);
                }
            }
            _ => {
                info!("Unknown packet");
            }
        }
    }
}
