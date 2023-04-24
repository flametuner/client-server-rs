use crate::{client::Location, packets};

use super::{Identifiable, Readable, Writeable};
use anyhow::Result;
use std::{
    io::{BufReader, Read, Write},
    net::TcpStream,
};

packets! {

    KeepAlivePacket {
    },
    LocationPacket {
        x f32;
        y f32;
    }

}

// #[derive(Debug)]
// pub struct KeepAlivePacket;
//
// impl Identifiable for KeepAlivePacket {
//     fn id(&self) -> u8 {
//         0x00
//     }
// }
//
// impl Readable for KeepAlivePacket {
//     fn read(_reader: &mut BufReader<&mut TcpStream>) -> Result<Self> {
//         Ok(Self)
//     }
// }
//
// impl Writeable for KeepAlivePacket {
//     fn write(&self, writer: &mut TcpStream) -> Result<()> {
//         writer.write_all(&self.id().to_le_bytes())?;
//         Ok(())
//     }
// }

// #[derive(Debug)]
// pub struct LocationPacket {
//     x: f32,
//     y: f32,
// }

impl LocationPacket {
    pub fn from(loc: Location) -> Self {
        Self { x: loc.x, y: loc.y }
    }
}

// impl Identifiable for LocationPacket {
//     fn id(&self) -> u8 {
//         0x02
//     }
// }

// impl Readable for LocationPacket {
//     fn read(reader: &mut BufReader<&mut TcpStream>) -> Result<Self> {
//         let mut buf = [0u8; 4];
//         reader.read_exact(&mut buf)?;
//         let x = f32::from_le_bytes(buf);
//         reader.read_exact(&mut buf)?;
//         let y = f32::from_le_bytes(buf);
//         Ok(Self { x, y })
//     }
// }
//
// impl Writeable for LocationPacket {
//     fn write(&self, writer: &mut TcpStream) -> Result<()> {
//         writer.write_all(&self.id().to_le_bytes())?;
//         writer.write_all(&self.x.to_le_bytes())?;
//         writer.write_all(&self.y.to_le_bytes())?;
//         Ok(())
//     }
// }
