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
pub struct SMSG_SPLINE_MOVE_SET_HOVER {
    pub guid: Guid,
}

impl ServerMessageWrite for SMSG_SPLINE_MOVE_SET_HOVER {}

impl MessageBody for SMSG_SPLINE_MOVE_SET_HOVER {
    const OPCODE: u16 = 0x0307;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: PackedGuid
        let guid = Guid::read_packed(r)?;

        Ok(Self {
            guid,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.write_packed(w)?;

        Ok(())
    }
}

impl VariableSized for SMSG_SPLINE_MOVE_SET_HOVER {
    fn size(&self) -> usize {
        self.guid.size() // guid: PackedGuid
    }
}

impl MaximumPossibleSized for SMSG_SPLINE_MOVE_SET_HOVER {
    fn maximum_possible_size() -> usize {
        9 // guid: PackedGuid
    }
}

