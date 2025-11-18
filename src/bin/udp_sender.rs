use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

use std::sync::Arc;

fn main() -> std::io::Result<()> {
    println!("UDP Sender - sending packets to TAP device");

    let socket = Arc::new(UdpSocket::bind("0.0.0.0:0")?);
    socket.set_read_timeout(Some(Duration::from_secs(1)))?;
    let socket_clone = socket.clone();
    let target = "192.168.10.2:8888";

    thread::spawn(move || {
        println!("Sending UDP packets to {}", target);

        for i in 1..=10 {
            let message = format!("Test message #{}", i);
            let _ = socket_clone.send_to(message.as_bytes(), target);
            println!("Sent: {}", message);

            thread::sleep(Duration::from_secs(1));
        }

        println!("Done!");
    });

    let mut buf = [0u8; 1024];
    loop {
        match socket.recv_from(&mut buf) {
            Ok((size, addr)) => {
                let data = &buf[..size];
                println!(
                    "Receviced from {}:{:?}",
                    addr,
                    String::from_utf8_lossy(data)
                )
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::TimedOut
                    || e.kind() == std::io::ErrorKind::WouldBlock
                {
                    println!("Timeout, exiting ...");
                    break;
                }
                eprintln!("Receive error: {}", e)
            }
        }
    }
    Ok(())
}
