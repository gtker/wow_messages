use std::convert::{TryFrom, TryInto};
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
pub struct SMSG_UPDATE_INSTANCE_OWNERSHIP {
    pub has_been_saved: u32,
}

impl ServerMessageWrite for SMSG_UPDATE_INSTANCE_OWNERSHIP {}

impl MessageBody for SMSG_UPDATE_INSTANCE_OWNERSHIP {
    const OPCODE: u16 = 0x032b;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // has_been_saved: u32
        let has_been_saved = crate::util::read_u32_le(r)?;

        Ok(Self {
            has_been_saved,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // has_been_saved: u32
        w.write_all(&self.has_been_saved.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_UPDATE_INSTANCE_OWNERSHIP {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_UPDATE_INSTANCE_OWNERSHIP {
    fn maximum_possible_size() -> usize {
        4 // has_been_saved: u32
    }
}

