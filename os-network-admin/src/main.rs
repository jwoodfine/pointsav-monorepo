#[cfg(feature = "daemon")]
mod fleet_watch;

fn main() -> std::io::Result<()> {
    #[cfg(feature = "daemon")]
    return fleet_watch::run();

    #[cfg(not(feature = "daemon"))]
    phase_s1_udp_loop()
}

// Phase S1 UDP telemetry loop — preserved for bare-metal seL4 OS mode.
// Replaced by fleet_watch::run() when compiled with --features daemon.
#[cfg(not(feature = "daemon"))]
fn phase_s1_udp_loop() -> std::io::Result<()> {
    use std::io::Write;
    use std::net::UdpSocket;
    use std::time::Duration;

    let peer_ip = "10.0.0.101:5000";
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_read_timeout(Some(Duration::from_secs(2)))?;

    eprintln!("os-network-admin: connecting to mesh node at {peer_ip}...");

    loop {
        socket.send_to(b"GET_VITALS", peer_ip)?;
        let mut buf = [0; 1024];
        match socket.recv_from(&mut buf) {
            Ok((amt, _)) => {
                let response = String::from_utf8_lossy(&buf[..amt]);
                print!("\rstatus: {}    ", response);
                std::io::stdout().flush().unwrap();
            }
            Err(_) => eprintln!("timeout: substrate link interrupted"),
        }
        std::thread::sleep(Duration::from_millis(1000));
    }
}
