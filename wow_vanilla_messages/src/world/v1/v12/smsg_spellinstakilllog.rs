use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_SPELLINSTAKILLLOG {
    pub target_guid: Guid,
    pub spell: u32,
}

impl ServerMessageWrite for SMSG_SPELLINSTAKILLLOG {}

impl MessageBody for SMSG_SPELLINSTAKILLLOG {
    const OPCODE: u16 = 0x032f;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // target_guid: Guid
        let target_guid = Guid::read(r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        Ok(Self {
            target_guid,
            spell,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // target_guid: Guid
        self.target_guid.write(w)?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_SPELLINSTAKILLLOG {}

impl MaximumPossibleSized for SMSG_SPELLINSTAKILLLOG {
    fn maximum_possible_size() -> usize {
        8 // target_guid: Guid
        + 4 // spell: u32
    }
}

