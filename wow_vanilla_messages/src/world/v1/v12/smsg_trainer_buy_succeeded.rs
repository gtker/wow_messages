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
pub struct SMSG_TRAINER_BUY_SUCCEEDED {
    pub guid: Guid,
    pub id: u32,
}

impl ServerMessageWrite for SMSG_TRAINER_BUY_SUCCEEDED {}

impl MessageBody for SMSG_TRAINER_BUY_SUCCEEDED {
    const OPCODE: u16 = 0x01b3;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // id: u32
        let id = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            id,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_TRAINER_BUY_SUCCEEDED {}

impl MaximumPossibleSized for SMSG_TRAINER_BUY_SUCCEEDED {
    fn maximum_possible_size() -> usize {
        8 // guid: Guid
        + 4 // id: u32
    }
}

