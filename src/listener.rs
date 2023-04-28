use self::{movement::MoveEvent, teleport::TeleportEvent};

pub mod movement;
pub mod teleport;

pub enum Event<'a, 'b> {
    Move(&'a mut MoveEvent<'b>),
    Teleport(&'a mut TeleportEvent<'b>),
}

pub trait Cancellable {
    fn set_cancelled(&mut self, cancelled: bool);
    fn is_cancelled(&self) -> bool;
}

pub trait EventHandler {
    fn on_move(&self, _event: &mut MoveEvent) {}
    fn on_teleport(&self, _event: &mut TeleportEvent) {}
}

pub struct MyListener;

impl EventHandler for MyListener {
    fn on_move(&self, event: &mut MoveEvent) {
        if event.from().distance(&event.to()) > 10f32 {
            event.set_cancelled(true);
            println!("Cancelled move event!");
        }
    }
}
