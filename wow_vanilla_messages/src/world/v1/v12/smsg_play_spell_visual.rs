use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ServerMessageWrite, MessageBody};
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
#[derive(Copy)]
pub struct SMSG_PLAY_SPELL_VISUAL {
    pub guid: Guid,
    pub spell_art_kit: u32,
}

impl ServerMessageWrite for SMSG_PLAY_SPELL_VISUAL {
    const OPCODE: u16 = 0x1f3;

    fn size_without_size_field(&self) -> u16 {
        Self::size() as u16
    }

}

impl MessageBody for SMSG_PLAY_SPELL_VISUAL {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // spell_art_kit: u32
        let spell_art_kit = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            spell_art_kit,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // spell_art_kit: u32
        w.write_all(&self.spell_art_kit.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_PLAY_SPELL_VISUAL {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_PLAY_SPELL_VISUAL {
    fn maximum_possible_size() -> usize {
        8 // guid: Guid
        + 4 // spell_art_kit: u32
    }
}

