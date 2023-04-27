use anyhow::Result;

use std::{
    io::{BufReader, Read, Write},
    net::TcpStream,
};

pub mod packets;

pub struct PacketId(u8);

impl Readable for PacketId {
    fn read(reader: &mut BufReader<&mut TcpStream>) -> Result<Self>
    where
        Self: Sized,
    {
        let mut buf = [0u8; 1];
        reader.read_exact(&mut buf)?;
        Ok(Self(buf[0]))
    }
}

impl Writeable for PacketId {
    fn write(&self, writer: &mut TcpStream) -> Result<()> {
        writer.write_all(&self.0.to_le_bytes())?;
        Ok(())
    }
}

trait Identifiable {
    fn id(&self) -> u8;
}

pub trait Readable {
    fn read(reader: &mut BufReader<&mut TcpStream>) -> Result<Self>
    where
        Self: Sized;
}

pub trait Writeable {
    fn write(&self, writer: &mut TcpStream) -> Result<()>;
}

impl Writeable for f32 {
    fn write(&self, writer: &mut TcpStream) -> Result<()> {
        writer.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl Readable for f32 {
    fn read(reader: &mut BufReader<&mut TcpStream>) -> Result<Self> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        Ok(f32::from_le_bytes(buf))
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
                        let $field = <$typ $(<$generics>)?>::read(reader)?;
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
                        self.$field.write(writer)?;
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
            pub fn id(&self) -> u8 {
                match self {
                    $(
                        $ident::$packet(_) => $id,
                    )*
                }
            }
        }

        impl crate::networking::Readable for $ident {
            fn read(reader: &mut BufReader<&mut TcpStream>) -> Result<Self>
                where
                    Self: Sized
                    {
                let packet_id = PacketId::read(reader)?.0;
                match packet_id {
                    $(
                        id if id == $id => Ok($ident::$packet($packet::read(reader)?)),
                    )*
                    _ => Err(anyhow::anyhow!("unknown packet ID {}", packet_id)),
                }
            }
        }

        impl crate::networking::Writeable for $ident {
            fn write(&self, writer: &mut TcpStream) -> anyhow::Result<()> {
                PacketId(self.id() as u8).write(writer)?;
                match self {
                    $(
                        $ident::$packet(packet) => {
                            packet.write(writer)?;
                        }
                    )*
                }
                Ok(())
            }
        }

    }
}
