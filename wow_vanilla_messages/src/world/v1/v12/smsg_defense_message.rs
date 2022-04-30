use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Map, MapError};
use crate::{WorldServerMessageWrite, MessageBody};
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
pub struct SMSG_DEFENSE_MESSAGE {
    pub map: Map,
    pub message_length: u32,
    pub message: String,
}

impl WorldServerMessageWrite for SMSG_DEFENSE_MESSAGE {
    const OPCODE: u16 = 0x33b;

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
impl MessageBody for SMSG_DEFENSE_MESSAGE {
    type Error = SMSG_DEFENSE_MESSAGEError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // map: Map
        let map = Map::read(r)?;

        // message_length: u32
        let message_length = crate::util::read_u32_le(r)?;

        // message: CString
        let message = crate::util::read_c_string_to_vec(r)?;
        let message = String::from_utf8(message)?;

        Ok(Self {
            map,
            message_length,
            message,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // map: Map
        self.map.write(w)?;

        // message_length: u32
        w.write_all(&self.message_length.to_le_bytes())?;

        // message: CString
        w.write_all(self.message.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
}

impl VariableSized for SMSG_DEFENSE_MESSAGE {
    fn size(&self) -> usize {
        Map::size() // map: Map
        + 4 // message_length: u32
        + self.message.len() + 1 // message: CString and Null Terminator
    }
}

impl MaximumPossibleSized for SMSG_DEFENSE_MESSAGE {
    fn maximum_possible_size() -> usize {
        Map::maximum_possible_size() // map: Map
        + 4 // message_length: u32
        + 256 // message: CString
    }
}

#[derive(Debug)]
pub enum SMSG_DEFENSE_MESSAGEError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
    Map(MapError),
}

impl std::error::Error for SMSG_DEFENSE_MESSAGEError {}
impl std::fmt::Display for SMSG_DEFENSE_MESSAGEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
            Self::Map(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_DEFENSE_MESSAGEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_DEFENSE_MESSAGEError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

impl From<MapError> for SMSG_DEFENSE_MESSAGEError {
    fn from(e: MapError) -> Self {
        Self::Map(e)
    }
}

