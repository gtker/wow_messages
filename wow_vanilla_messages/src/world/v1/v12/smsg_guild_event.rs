use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{GuildEvent, GuildEventError};
use crate::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use crate::AsyncReadWrite;
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_GUILD_EVENT {
    pub event: GuildEvent,
    pub event_descriptions: Vec<String>,
}

impl WorldServerMessageWrite for SMSG_GUILD_EVENT {
    const OPCODE: u16 = 0x92;

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
impl WorldMessageBody for SMSG_GUILD_EVENT {
    type Error = SMSG_GUILD_EVENTError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // event: GuildEvent
        let event = GuildEvent::read(r)?;

        // amount_of_events: u8
        let amount_of_events = crate::util::read_u8_le(r)?;

        // event_descriptions: CString[amount_of_events]
        let mut event_descriptions = Vec::with_capacity(amount_of_events as usize);
        for i in 0..amount_of_events {
            let s = crate::util::read_c_string_to_vec(r)?;
            event_descriptions.push(String::from_utf8(s)?);
        }

        Ok(Self {
            event,
            event_descriptions,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // event: GuildEvent
        self.event.write(w)?;

        // amount_of_events: u8
        w.write_all(&(self.event_descriptions.len() as u8).to_le_bytes())?;

        // event_descriptions: CString[amount_of_events]
        for i in self.event_descriptions.iter() {
            w.write_all(&i.as_bytes())?;
            w.write_all(&[0])?;
        }

        Ok(())
    }
}

impl VariableSized for SMSG_GUILD_EVENT {
    fn size(&self) -> usize {
        GuildEvent::size() // event: GuildEvent
        + 1 // amount_of_events: u8
        + self.event_descriptions.iter().fold(0, |acc, x| acc + x.len() + 1) // event_descriptions: CString[amount_of_events]
    }
}

impl MaximumPossibleSized for SMSG_GUILD_EVENT {
    fn maximum_possible_size() -> usize {
        GuildEvent::maximum_possible_size() // event: GuildEvent
        + 1 // amount_of_events: u8
        + 255 * 256 // event_descriptions: CString[amount_of_events]
    }
}

#[derive(Debug)]
pub enum SMSG_GUILD_EVENTError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
    GuildEvent(GuildEventError),
}

impl std::error::Error for SMSG_GUILD_EVENTError {}
impl std::fmt::Display for SMSG_GUILD_EVENTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
            Self::GuildEvent(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_GUILD_EVENTError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_GUILD_EVENTError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

impl From<GuildEventError> for SMSG_GUILD_EVENTError {
    fn from(e: GuildEventError) -> Self {
        Self::GuildEvent(e)
    }
}

