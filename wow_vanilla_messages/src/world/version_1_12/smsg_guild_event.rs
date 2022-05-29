use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::GuildEvent;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_GUILD_EVENT {
    pub event: GuildEvent,
    pub event_descriptions: Vec<String>,
}

impl ServerMessage for SMSG_GUILD_EVENT {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // event: GuildEvent
        w.write_all(&(self.event.as_int() as u8).to_le_bytes())?;

        // amount_of_events: u8
        w.write_all(&(self.event_descriptions.len() as u8).to_le_bytes())?;

        // event_descriptions: CString[amount_of_events]
        for i in self.event_descriptions.iter() {
            w.write_all(i.as_bytes())?;
            w.write_all(&[0])?;
        }

        Ok(())
    }
    const OPCODE: u16 = 0x0092;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // event: GuildEvent
        let event: GuildEvent = crate::util::read_u8_le(r)?.try_into()?;

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

}

impl SMSG_GUILD_EVENT {
    pub(crate) fn size(&self) -> usize {
        1 // event: GuildEvent
        + 1 // amount_of_events: u8
        + self.event_descriptions.iter().fold(0, |acc, x| acc + x.len() + 1) // event_descriptions: CString[amount_of_events]
    }
}

