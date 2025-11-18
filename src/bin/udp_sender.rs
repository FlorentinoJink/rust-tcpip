use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    println!("UDP Sender - sending packets to TAP device");
    
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let target = "192.168.10.2:8888";
    
    println!("Sending UDP packets to {}", target);
    
    for i in 1..=10 {
        let message = format!("Test message #{}", i);
        socket.send_to(message.as_bytes(), target)?;
        println!("Sent: {}", message);
        thread::sleep(Duration::from_secs(1));
    }
    
    println!("Done!");
    Ok(())
}
