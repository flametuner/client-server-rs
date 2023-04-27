use crate::networking::{
    packets::{Move, Teleport},
    Readable, Writeable,
};
use anyhow::Result;
use std::{
    io::BufReader,
    net::{Shutdown, TcpStream},
    sync::{
        mpsc::{self, Sender},
        Arc, Mutex,
    },
    thread,
};
use uuid::Uuid;

use crate::{networking::packets::Packet, server::Server};

pub struct Player {
    pub id: Uuid,
    packet_sender: Sender<Packet>,
    location: Location,
    socket: TcpStream,
    server: Arc<Mutex<Server>>,
}

#[derive(Clone, Copy, Default)]
pub struct Location {
    pub x: f32,
    pub y: f32,
}

impl Player {
    pub fn new(stream: TcpStream, server: Arc<Mutex<Server>>) -> Result<Arc<Mutex<Self>>> {
        let id = Uuid::new_v4();
        println!("[Client {id}] Connection established!");
        let (tx, rx) = mpsc::channel::<Packet>();

        let client = Self {
            id,
            packet_sender: tx,
            socket: stream.try_clone()?,
            location: Location::default(),
            server,
        };

        let client = Arc::new(Mutex::new(client));

        {
            let client = client.clone();
            let mut stream = stream.try_clone()?;
            stream.set_nonblocking(false)?;
            thread::spawn(move || {
                loop {
                    let mut reader = BufReader::new(&mut stream);
                    match Packet::read(&mut reader) {
                        Ok(packet) => {
                            // Handle Packet
                            // println!("[Client {id}] Packet received from {id}: {:?}", packet);
                            // TODO send to listeners
                            match packet {
                                Packet::Move(Move { x, y }) => {
                                    client.lock().unwrap().walk(x, y);
                                }
                                _ => {}
                            }
                        }
                        Err(e) => {
                            println!("Error reading packet: {:?}", e);
                            client.lock().unwrap().disconnect();
                            break;
                        }
                    }
                }
            });
        }
        {
            let mut stream = stream;
            let client = client.clone();
            thread::spawn(move || loop {
                let packet = rx.recv().unwrap();

                // write to stream
                match packet.write(&mut stream) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("Error sending packet: {:?}", e);
                        client.lock().unwrap().disconnect();
                        break;
                    }
                }
            });
        }
        Ok(client)
    }

    pub fn get_location(&self) -> Location {
        self.location
    }

    pub fn teleport(&mut self, location: Location) {
        self.location = location;
        self.send_packet(Packet::Teleport(Teleport::from(location)));
    }

    pub fn walk(&mut self, x: f32, y: f32) {
        self.location.x += x;
        self.location.y += y;
        println!(
            "Client location: x:{}, y:{}",
            self.location.x, self.location.y
        );
    }

    pub fn disconnect(&mut self) {
        let id = self.id;
        println!("[Client {id}] An error ocourred. Shutting down");
        match self.socket.shutdown(Shutdown::Both) {
            Ok(_) => {}
            Err(_) => {}
        }
        self.server.lock().unwrap().disconnect_client(&id);
    }

    pub fn send_packet(&mut self, packet: Packet) {
        println!("[Client {}] Sending packet: {:?}", self.id, packet);
        self.packet_sender.send(packet).unwrap();
    }
}
