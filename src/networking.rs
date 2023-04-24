use anyhow::Result;

use std::{
    io::{BufReader, Read},
    net::TcpStream,
};

use self::packets::{KeepAlivePacket, LocationPacket};

pub mod packets;

// use phf::phf_map;
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct VarInt(pub i32);

trait Identifiable {
    fn id(&self) -> u8;
}

trait Readable {
    fn read(reader: &mut BufReader<&mut TcpStream>) -> Result<Self>
    where
        Self: Sized;
}

pub trait Writeable {
    fn write(&self, writer: &mut TcpStream) -> Result<()>;
}

#[derive(Debug)]
pub enum Packet {
    KeepAlive(KeepAlivePacket),
    Teleport(LocationPacket),
    Move(LocationPacket),
}

impl Packet {
    pub fn from(stream: &mut TcpStream) -> Result<Self> {
        let mut reader = BufReader::new(stream);
        let mut buf = [0u8; 1];
        reader.read_exact(&mut buf)?;
        let id = buf[0];
        match id {
            0x00 => Ok(Packet::KeepAlive(KeepAlivePacket {})),
            0x01 => Ok(Packet::Teleport(LocationPacket::read(&mut reader)?)),
            0x02 => Ok(Packet::Move(LocationPacket::read(&mut reader)?)),
            _ => Err(anyhow::anyhow!("Invalid packet id: {}", id)),
        }
    }
}

impl Writeable for Packet {
    fn write(&self, writer: &mut TcpStream) -> Result<()> {
        match self {
            Packet::KeepAlive(packet) => packet.write(writer),
            Packet::Teleport(packet) => packet.write(writer),
            Packet::Move(packet) => packet.write(writer),
        }
    }
}

#[macro_export]
macro_rules! packets {
    (
        $(
            $packet:ident {
                $(
                    $field:ident $typ:ident $(<$generics:ident>)?
                );* $(;)?
            } $(,)?
        )*
    ) => {
        $(
            #[derive(Debug, Clone)]
            pub struct $packet {
                $(
                    pub $field: $typ $(<$generics>)?,
                )*
            }


            #[allow(unused_imports, unused_variables)]
            impl crate::networking::Readable for $packet {
                fn read(reader: &mut BufReader<&mut TcpStream>) -> anyhow::Result<Self>
                where
                    Self: Sized
                {
                    $(
                        let $field = <$typ $(<$generics>)?>::read(buffer)
                            .context(concat!("failed to read field `", stringify!($field), "` of packet `", stringify!($packet), "`"))?
                            .into();
                    )*

                    Ok(Self {
                        $(
                            $field,
                        )*
                    })
                }
            }

            #[allow(unused_variables)]
            impl crate::networking::Writeable for $packet {
                fn write(&self, writer: &mut TcpStream) -> anyhow::Result<()> {
                    $(
                        user_type_convert_to_writeable!($typ $(<$generics>)?, &self.$field).write(writer)?;
                    )*
                    Ok(())
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! packet_enum {
    (
        $ident:ident {
            $($id:literal = $packet:ident),* $(,)?
        }
    ) => {
        #[derive(Debug, Clone)]
        pub enum $ident {
            $(
                $packet($packet),
            )*
        }

        impl $ident {
            /// Returns the packet ID of this packet.
            pub fn id(&self) -> u32 {
                match self {
                    $(
                        $ident::$packet(_) => $id,
                    )*
                }
            }
        }

        impl crate::Readable for $ident {
            fn read(buffer: &mut ::std::io::Cursor<&[u8]>, version: crate::ProtocolVersion) -> anyhow::Result<Self>
            where
                Self: Sized
            {
                let packet_id = VarInt::read(buffer, version)?.0;
                match packet_id {
                    $(
                        id if id == $id => Ok($ident::$packet($packet::read(buffer, version)?)),
                    )*
                    _ => Err(anyhow::anyhow!("unknown packet ID {}", packet_id)),
                }
            }
        }

        impl crate::Writeable for $ident {
            fn write(&self, buffer: &mut Vec<u8>, version: crate::ProtocolVersion) -> anyhow::Result<()> {
                VarInt(self.id() as i32).write(buffer, version)?;
                match self {
                    $(
                        $ident::$packet(packet) => {
                            packet.write(buffer, version)?;
                        }
                    )*
                }
                Ok(())
            }
        }

        $(
            impl VariantOf<$ident> for $packet {
                fn discriminant_id() -> u32 { $id }

                #[allow(unreachable_patterns)]
                fn destructure(e: $ident) -> Option<Self> {
                    match e {
                        $ident::$packet(p) => Some(p),
                        _ => None,
                    }
                }
            }

            impl From<$packet> for $ident {
                fn from(packet: $packet) -> Self {
                    $ident::$packet(packet)
                }
            }
        )*
    }
}
