use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Map, MapError};
use crate::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use crate::AsyncReadWrite;
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_TRANSFER_PENDING {
    pub map: Map,
    pub has_transport: Option<SMSG_TRANSFER_PENDING_has_transport>,
}

impl WorldServerMessageWrite for SMSG_TRANSFER_PENDING {
    const OPCODE: u16 = 0x3f;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (self.size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (self.size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_TRANSFER_PENDING {
    type Error = SMSG_TRANSFER_PENDINGError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // map: Map
        let map = Map::read(r)?;

        // optional has_transport
        let current_size = {
            0 // If no fields are present, TODO remove when not needed
            + Map::size() // map: Map
        };
        let has_transport = if current_size < body_size as usize {
            // transport: u32
            let transport = crate::util::read_u32_le(r)?;

            // transport_map: Map
            let transport_map = Map::read(r)?;

            Some(SMSG_TRANSFER_PENDING_has_transport {
                transport,
                transport_map,
            })
        } else {
            None
        };

        Ok(Self {
            map,
            has_transport,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // map: Map
        self.map.write(w)?;

        // optional has_transport
        if let Some(v) = &self.has_transport {
            // transport: u32
            w.write_all(&v.transport.to_le_bytes())?;

            // transport_map: Map
            v.transport_map.write(w)?;

        }

        Ok(())
    }
}

impl VariableSized for SMSG_TRANSFER_PENDING {
    fn size(&self) -> usize {
        Map::size() // map: Map
        + {
            if let Some(v) = &self.has_transport {
                v.size()
            } else {
                0
            }
        } // optional has_transport
    }
}

impl MaximumPossibleSized for SMSG_TRANSFER_PENDING {
    fn maximum_possible_size() -> usize {
        Map::maximum_possible_size() // map: Map
        + 65536 // optional has_transport
    }
}

#[derive(Debug)]
pub enum SMSG_TRANSFER_PENDINGError {
    Io(std::io::Error),
    Map(MapError),
}

impl std::error::Error for SMSG_TRANSFER_PENDINGError {}
impl std::fmt::Display for SMSG_TRANSFER_PENDINGError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Map(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_TRANSFER_PENDINGError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<MapError> for SMSG_TRANSFER_PENDINGError {
    fn from(e: MapError) -> Self {
        Self::Map(e)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SMSG_TRANSFER_PENDING_has_transport {
    pub transport: u32,
    pub transport_map: Map,
}

impl SMSG_TRANSFER_PENDING_has_transport {
    pub fn size(&self) -> usize {
        4 // transport: u32
        + Map::size() // transport_map: Map
    }
}

