use crate::client::{Location, Player};

pub trait Event {}

pub struct MoveEvent {
    cancelled: bool,
    player: Box<Player>,
    from: Location,
    to: Location,
}

impl Event for MoveEvent {}

pub trait Listener {}

pub trait EventHandler<E: Event> {
    fn handle(&self, event: &mut E);
}

pub struct OnMove;

impl EventHandler<MoveEvent> for OnMove {
    fn handle(&self, event: &mut MoveEvent) {}
}
