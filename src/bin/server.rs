use anyhow::Result;
use client_server_rs::{client::Player, server::Server};
use std::{
    net::TcpListener,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

fn main() -> Result<()> {
    start_server()?;

    Ok(())
}

fn start_server() -> Result<()> {
    println!("Starting server...");
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    let server = Arc::new(Mutex::new(Server::default()));

    // we want to look for incoming request on each tick
    listener.set_nonblocking(true)?;
    let tick = Duration::from_secs_f64(1f64 / 20f64); // 20 ticks per seconds
    let mut latest_tick = std::time::Instant::now();
    loop {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    server
                        .lock()
                        .unwrap()
                        .add_client(Player::new(stream, server.clone())?);
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                Err(_) => println!("Error"),
            }
        }
        {
            server.lock().unwrap().tick();
        }

        // tick time
        let elapsed = latest_tick.elapsed();
        if tick > elapsed {
            thread::sleep(tick - elapsed);
        }
        latest_tick = std::time::Instant::now();
    }
}
