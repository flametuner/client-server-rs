use getset::{Getters, Setters};

use crate::client::{Location, Player};

use super::Cancellable;

#[derive(Getters, Setters)]
pub struct TeleportEvent<'a> {
    cancelled: bool,
    #[getset(get = "pub")]
    player: &'a Player,
    #[getset(get = "pub", set = "pub")]
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
    pub fn new(player: &'a Player, to: Location) -> Self {
        Self {
            cancelled: false,
            player,
            to,
        }
    }
}
