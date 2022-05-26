use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_PLAY_SPELL_IMPACT {
    pub guid: Guid,
    pub spell_visual_kit: u32,
}

impl SMSG_PLAY_SPELL_IMPACT {
    pub(crate) fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // spell_visual_kit: u32
        w.write_all(&self.spell_visual_kit.to_le_bytes())?;

        Ok(())
    }
}

impl ServerMessage for SMSG_PLAY_SPELL_IMPACT {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // spell_visual_kit: u32
        w.write_all(&self.spell_visual_kit.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x01f7;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        12
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // spell_visual_kit: u32
        let spell_visual_kit = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            spell_visual_kit,
        })
    }

}

