use crate::networking::PacketId;
use crate::{client::Location, packet_enum, packets};

use anyhow::Result;
use std::{io::BufReader, net::TcpStream};

packet_enum! {
    Packet {
        0x00 = KeepAlive,
        0x01 = Move,
        0x02 = Teleport,
    }
}

packets! {
    KeepAlive{
    },
    Move{
        x f32;
        y f32;
    },
    Teleport{
        x f32;
        y f32;
    }

}

impl From<Location> for Move {
    fn from(loc: Location) -> Self {
        Self { x: loc.x, y: loc.y }
    }
}

impl From<Location> for Teleport {
    fn from(loc: Location) -> Self {
        Self { x: loc.x, y: loc.y }
    }
}
