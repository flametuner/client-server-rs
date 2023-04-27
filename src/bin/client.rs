use std::{
    io::{self, BufReader},
    net::TcpStream,
    thread,
    time::{Duration, Instant},
};

use anyhow::Result;
use client_server_rs::networking::{
    packets::{KeepAlive, Move, Packet},
    Readable, Writeable,
};

fn main() -> Result<()> {
    start_client()?;
    Ok(())
}

const KEEP_ALIVE_DURATION: Duration = Duration::from_secs(30);

// fn start_client() -> Result<JoinHandle<_>> {
fn start_client() -> Result<()> {
    let mut socket = TcpStream::connect("127.0.0.1:7878")?;
    println!("[Client] Connected to server");
    // let mut writer = BufWriter::new(&mut socket);

    let mut latest_keep_alive = Instant::now();
    let tick = Duration::from_secs_f64(1f64 / 20f64); // 20 ticks per seconds
    let mut latest_tick = std::time::Instant::now();
    socket.set_nonblocking(true)?;
    loop {
        let mut bytes = [0u8; 1];
        match socket.peek(&mut bytes) {
            Ok(b) if b > 0 => {
                socket.set_nonblocking(false)?;
                let mut reader = BufReader::new(&mut socket);

                let packet = Packet::read(&mut reader)?;
                println!("[Client] Packet received: {:?}", packet);
                // match Packet::read(&mut reader) {
                //     Ok(packet) => {
                //         println!("[Client] Packet received: {:?}", packet);
                //     }
                //     Err(e) => {
                //         println!("[Client] Error reading packet: {:?}", e);
                //     }
                // }
                socket.set_nonblocking(true)?;
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {}
            Ok(_) | Err(_) => {
                println!("[Client] Error with connection. Shutting down");
                break;
            }
        }

        if rand::random::<f32>() > 0.009 {
            let x =
                if rand::random::<bool>() { 1f32 } else { -1f32 } * rand::random::<f32>() * 10f32;
            let y =
                if rand::random::<bool>() { 1f32 } else { -1f32 } * rand::random::<f32>() * 10f32;
            let packet = Packet::Move(Move { x, y });
            // packet.write(&mut socket)?;
            match packet.write(&mut socket) {
                Ok(_) => {}
                Err(e) => {
                    println!("Error sending keep alive: {:?}", e);
                }
            }
            // println!("Sent: {:?}", packet);
        }

        if latest_keep_alive.elapsed() > KEEP_ALIVE_DURATION {
            let packet = Packet::KeepAlive(KeepAlive {});
            match packet.write(&mut socket) {
                Ok(_) => {}
                Err(e) => {
                    println!("Error sending keep alive: {:?}", e);
                }
            }
            println!("Sent: {:?}", packet);
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
