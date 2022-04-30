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
pub struct SMSG_CLIENT_CONTROL_UPDATE {
    pub guid: Guid,
    pub allow_movement: u8,
}

impl ServerMessageWrite for SMSG_CLIENT_CONTROL_UPDATE {}

impl MessageBody for SMSG_CLIENT_CONTROL_UPDATE {
    const OPCODE: u16 = 0x0159;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: PackedGuid
        let guid = Guid::read_packed(r)?;

        // allow_movement: u8
        let allow_movement = crate::util::read_u8_le(r)?;

        Ok(Self {
            guid,
            allow_movement,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.write_packed(w)?;

        // allow_movement: u8
        w.write_all(&self.allow_movement.to_le_bytes())?;

        Ok(())
    }
}

impl VariableSized for SMSG_CLIENT_CONTROL_UPDATE {
    fn size(&self) -> usize {
        self.guid.size() // guid: PackedGuid
        + 1 // allow_movement: u8
    }
}

impl MaximumPossibleSized for SMSG_CLIENT_CONTROL_UPDATE {
    fn maximum_possible_size() -> usize {
        9 // guid: PackedGuid
        + 1 // allow_movement: u8
    }
}

