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
pub struct SMSG_CLEAR_COOLDOWN {
    pub id: u32,
    pub target_guid: Guid,
}

impl ServerMessageWrite for SMSG_CLEAR_COOLDOWN {}

impl MessageBody for SMSG_CLEAR_COOLDOWN {
    const OPCODE: u16 = 0x01de;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // id: u32
        let id = crate::util::read_u32_le(r)?;

        // target_guid: Guid
        let target_guid = Guid::read(r)?;

        Ok(Self {
            id,
            target_guid,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // target_guid: Guid
        self.target_guid.write(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_CLEAR_COOLDOWN {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_CLEAR_COOLDOWN {
    fn maximum_possible_size() -> usize {
        4 // id: u32
        + 8 // target_guid: Guid
    }
}

