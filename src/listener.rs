use crate::client::{Location, Player};

pub enum Event<'a> {
    Move(MoveEvent<'a>),
    Teleport(TeleportEvent<'a>),
}

impl<'a> Event<'a> {}

pub trait Cancellable {
    fn set_cancelled(&mut self, cancelled: bool);
    fn is_cancelled(&self) -> bool;
}

pub struct TeleportEvent<'a> {
    cancelled: bool,
    player: &'a Player,
    to: Location,
}

impl Cancellable for TeleportEvent<'_> {
    fn set_cancelled(&mut self, cancelled: bool) {
        self.cancelled = cancelled;
    }

    fn is_cancelled(&self) -> bool {
        self.cancelled
    }
}

impl<'a> TeleportEvent<'a> {
    pub fn set_to(&mut self, to: Location) {
        self.to = to;
    }

    pub fn get_player(&self) -> &Player {
        &self.player
    }

    pub fn get_to(&self) -> Location {
        self.to
    }
}

pub struct MoveEvent<'a> {
    cancelled: bool,
    player: &'a Player,
    from: Location,
    to: Location,
}

impl Cancellable for MoveEvent<'_> {
    fn set_cancelled(&mut self, cancelled: bool) {
        self.cancelled = cancelled;
    }

    fn is_cancelled(&self) -> bool {
        self.cancelled
    }
}

impl<'a> MoveEvent<'a> {
    pub fn new(player: &'a Player, to: Location) -> Self {
        Self {
            cancelled: false,
            player,
            from: player.get_location(),
            to,
        }
    }

    pub fn get_player(&self) -> &Player {
        &self.player
    }

    pub fn get_from(&self) -> &Location {
        &self.from
    }

    pub fn get_to(&self) -> Location {
        self.to
    }

    pub fn set_to(&mut self, to: Location) {
        self.to = to;
    }
}

pub trait EventHandler {
    fn on_move(&self, _event: &mut MoveEvent) {}
    fn on_teleport(&self, _event: &mut TeleportEvent) {}
}

pub struct MyListener;

impl EventHandler for MyListener {
    fn on_move(&self, event: &mut MoveEvent) {
        if event.get_from().distance(&event.get_to()) > 10f32 {
            event.set_cancelled(true);
            println!("Cancelled move event!");
        }
    }
}
