use std::convert::{TryFrom, TryInto};
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
pub struct SMSG_SET_FLAT_SPELL_MODIFIER {
    pub eff: u8,
    pub op: u8,
    pub value: u32,
}

impl ServerMessageWrite for SMSG_SET_FLAT_SPELL_MODIFIER {}

impl MessageBody for SMSG_SET_FLAT_SPELL_MODIFIER {
    const OPCODE: u16 = 0x0266;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // eff: u8
        let eff = crate::util::read_u8_le(r)?;

        // op: u8
        let op = crate::util::read_u8_le(r)?;

        // value: u32
        let value = crate::util::read_u32_le(r)?;

        Ok(Self {
            eff,
            op,
            value,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // eff: u8
        w.write_all(&self.eff.to_le_bytes())?;

        // op: u8
        w.write_all(&self.op.to_le_bytes())?;

        // value: u32
        w.write_all(&self.value.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_SET_FLAT_SPELL_MODIFIER {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_SET_FLAT_SPELL_MODIFIER {
    fn maximum_possible_size() -> usize {
        1 // eff: u8
        + 1 // op: u8
        + 4 // value: u32
    }
}

