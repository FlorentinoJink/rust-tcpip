pub mod error;
pub mod ethernet;
pub mod arp;
pub mod icmp;
pub mod ip;
pub mod socket;
pub mod udp;

pub fn init_tracing() {
    use tracing_subscriber::fmt;

    fmt()
        //.with_max_level(tracing::Level::INFO)
        .with_target(true)
        .with_line_number(true)
        .init();
}

#[cfg(test)]
mod tests {}
