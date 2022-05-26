use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::Emote;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_EMOTE {
    pub emote: Emote,
    pub guid: Guid,
}

impl SMSG_EMOTE {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 12], std::io::Error> {
        let mut array_w = [0u8; 12];
        let mut w = array_w.as_mut_slice();
        // emote: Emote
        w.write_all(&(self.emote.as_int() as u32).to_le_bytes())?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(array_w)
    }
}

impl ServerMessage for SMSG_EMOTE {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // emote: Emote
        w.write_all(&(self.emote.as_int() as u32).to_le_bytes())?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0103;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        12
    }

    type Error = crate::errors::ParseError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // emote: Emote
        let emote: Emote = crate::util::read_u32_le(r)?.try_into()?;

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            emote,
            guid,
        })
    }

}

