use crate::{
    listener::{movement::MoveEvent, Cancellable, Event},
    networking::{packets::*, Readable, Writeable},
};
use anyhow::Result;
use std::{
    io::BufReader,
    net::{Shutdown, TcpStream},
    ops::{Add, Sub},
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

impl Location {
    pub fn distance(&self, other: &Self) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}
impl Add for Location {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Location {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
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
            thread::spawn(move || loop {
                let mut reader = BufReader::new(&mut stream);
                match Packet::read(&mut reader) {
                    Ok(packet) => client.lock().unwrap().process_packet(packet),
                    Err(e) => {
                        println!("Error reading packet: {:?}", e);
                        client.lock().unwrap().disconnect();
                        break;
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

    fn process_packet(&mut self, packet: Packet) {
        match packet {
            Packet::Move(Move { x, y }) => {
                self.walk(x, y);
            }
            _ => {}
        }
    }

    pub fn get_location(&self) -> Location {
        self.location
    }

    pub fn teleport(&mut self, location: Location) {
        self.location = location;
        self.send_packet(Packet::Teleport(Teleport::from(location)));
    }

    pub fn walk(&mut self, x: f32, y: f32) {
        let to_location = self.location + Location { x, y };

        let mut event = MoveEvent::new(&self, to_location);
        {
            self.server
                .lock()
                .unwrap()
                .dispatch_event(Event::Move(&mut event));
        }

        if event.is_cancelled() {
            return;
        }

        self.location = event.to().clone();
        println!(
            "[Client {}] Moved to ({}, {})",
            self.id, self.location.x, self.location.y
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
