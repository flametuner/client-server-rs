use std::{
    io,
    net::TcpStream,
    thread,
    time::{Duration, Instant},
};

use anyhow::Result;
use test_rs::networking::{packets::KeepAlivePacket, Packet, Writeable};

fn main() -> Result<()> {
    start_client()?;
    Ok(())
}
// fn start_client() -> Result<JoinHandle<_>> {
fn start_client() -> Result<()> {
    let mut socket = TcpStream::connect("127.0.0.1:7878")?;
    println!("[Client] Connected to server");
    // let mut writer = BufWriter::new(&mut socket);

    let mut latest_keep_alive = Instant::now();
    let tick = Duration::from_secs_f64(1f64 / 20f64); // 20 ticks per seconds
    let mut latest_tick = std::time::Instant::now();
    loop {
        let mut bytes = [0u8; 1];
        socket.set_nonblocking(true)?;
        match socket.peek(&mut bytes) {
            Ok(b) if b > 0 => {
                let packet = Packet::from(&mut socket)?;
                println!("[Client] Received packet: {:?}", packet);
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {}
            Ok(_) | Err(_) => {
                println!("[Client] Error with connection. Shutting down");
                break;
            }
        }

        if latest_keep_alive.elapsed() > Duration::from_secs(10) {
            let packet = Packet::KeepAlive(KeepAlivePacket);
            packet.write(&mut socket)?;
            latest_keep_alive = Instant::now();
        }

        // tick time
        let elapsed = latest_tick.elapsed();
        if tick > elapsed {
            thread::sleep(tick - elapsed);
        }
        latest_tick = std::time::Instant::now();
    }
    Ok(())
}
