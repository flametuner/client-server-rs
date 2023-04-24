use crate::networking::PacketId;
use crate::{client::Location, packet_enum, packets};

use anyhow::Result;
use std::{io::BufReader, net::TcpStream};

packet_enum! {
    Packet {
        0x00 = KeepAlive(KeepAlivePacket),
        0x01 = Move(LocationPacket),
        0x02 = Teleport(LocationPacket),
    }
}

packets! {

    KeepAlivePacket {
    },
    LocationPacket {
        x f32;
        y f32;
    }

}

impl LocationPacket {
    pub fn from(loc: Location) -> Self {
        Self { x: loc.x, y: loc.y }
    }
}
