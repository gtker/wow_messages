use std::convert::{TryFrom, TryInto};
use crate::{ClientMessageWrite, MessageBody};
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
pub struct CMSG_QUESTLOG_REMOVE_QUEST {
    pub slot: u8,
}

impl ClientMessageWrite for CMSG_QUESTLOG_REMOVE_QUEST {}

impl MessageBody for CMSG_QUESTLOG_REMOVE_QUEST {
    const OPCODE: u16 = 0x0194;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // slot: u8
        let slot = crate::util::read_u8_le(r)?;

        Ok(Self {
            slot,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // slot: u8
        w.write_all(&self.slot.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for CMSG_QUESTLOG_REMOVE_QUEST {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_QUESTLOG_REMOVE_QUEST {
    fn maximum_possible_size() -> usize {
        1 // slot: u8
    }
}

