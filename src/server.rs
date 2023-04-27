use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use uuid::Uuid;

use crate::{
    client::Player,
    listener::{Event, EventHandler},
};

pub trait Listener {}

#[derive(Default)]
pub struct Server {
    clients: HashMap<Uuid, Arc<Mutex<Player>>>,
    listeners: Vec<Box<dyn EventHandler<dyn Event> + Sync + Send>>,
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

    pub fn add_listener(&mut self, listener: Box<dyn EventHandler<dyn Event> + Sync + Send>) {
        self.listeners.push(listener);
    }

    pub fn disconnect_client(&mut self, client_id: &Uuid) {
        self.clients.remove(client_id);
    }

    pub fn client_count(&self) -> usize {
        self.clients.len()
    }

    pub fn handle_event(&self, event: &impl Event) {
        for listener in &self.listeners {
            listener.handle(event);
        }
    }

    pub fn tick(&mut self) {
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
