use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::Emote;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_EMOTE {
    pub emote: Emote,
}

impl CMSG_EMOTE {
    pub(crate) fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // emote: Emote
        w.write_all(&(self.emote.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
}

impl ClientMessage for CMSG_EMOTE {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // emote: Emote
        w.write_all(&(self.emote.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0102;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        4
    }

    type Error = crate::errors::ParseError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // emote: Emote
        let emote: Emote = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            emote,
        })
    }

}

