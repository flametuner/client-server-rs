use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use uuid::Uuid;

use crate::{
    client::Player,
    listener::{
        Event::{self, *},
        EventHandler,
    },
};

type Handler = Box<dyn EventHandler + Sync + Send>;

#[derive(Default)]
pub struct Server {
    clients: HashMap<Uuid, Arc<Mutex<Player>>>,
    listeners: Vec<Handler>, // listeners: Vec<Box<dyn EventHandler<dyn Event>>>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn start() {}

    pub fn add_client(&mut self, client: Arc<Mutex<Player>>) {
        let id = client.lock().unwrap().id;
        self.clients.insert(id, client);
    }

    pub fn add_listener(&mut self, listener: Handler) {
        self.listeners.push(listener);
    }

    pub fn disconnect_client(&mut self, client_id: &Uuid) {
        self.clients.remove(client_id);
    }

    pub fn client_count(&self) -> usize {
        self.clients.len()
    }

    pub fn dispatch_event(&self, mut event: Event) {
        self.listeners.iter().for_each(|listener| match &mut event {
            Move(event) => listener.on_move(event),
            Teleport(event) => listener.on_teleport(event),
        });
    }

    pub fn tick(&mut self) {
        // physics

        // if !self.clients.is_empty() && rand::random::<f64>() > 0.995 {
        //     println!("Teleport");
        //     for client in &mut self.clients.values() {
        //         client.lock().unwrap().teleport(Location {
        //             x: rand::random::<f32>(),
        //             y: rand::random::<f32>(),
        //         });
        //     }
        // }
    }
}
