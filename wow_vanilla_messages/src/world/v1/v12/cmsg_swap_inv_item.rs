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
pub struct CMSG_SWAP_INV_ITEM {
    pub source_slot: u8,
    pub destination_slot: u8,
}

impl ClientMessageWrite for CMSG_SWAP_INV_ITEM {
    const OPCODE: u32 = 0x10d;

    fn size_without_size_field(&self) -> u16 {
        Self::size() as u16
    }

}

impl MessageBody for CMSG_SWAP_INV_ITEM {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // source_slot: u8
        let source_slot = crate::util::read_u8_le(r)?;

        // destination_slot: u8
        let destination_slot = crate::util::read_u8_le(r)?;

        Ok(Self {
            source_slot,
            destination_slot,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // source_slot: u8
        w.write_all(&self.source_slot.to_le_bytes())?;

        // destination_slot: u8
        w.write_all(&self.destination_slot.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for CMSG_SWAP_INV_ITEM {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_SWAP_INV_ITEM {
    fn maximum_possible_size() -> usize {
        1 // source_slot: u8
        + 1 // destination_slot: u8
    }
}

