use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{BuybackSlot, BuybackSlotError};
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
pub struct CMSG_BUYBACK_ITEM {
    pub guid: Guid,
    pub slot: BuybackSlot,
}

impl ClientMessageWrite for CMSG_BUYBACK_ITEM {
    const OPCODE: u32 = 0x290;

    fn size_without_size_field(&self) -> u16 {
        Self::size() as u16
    }

}

impl MessageBody for CMSG_BUYBACK_ITEM {
    type Error = CMSG_BUYBACK_ITEMError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // slot: BuybackSlot
        let slot = BuybackSlot::read(r)?;

        Ok(Self {
            guid,
            slot,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // slot: BuybackSlot
        self.slot.write(w)?;

        Ok(())
    }
}

impl ConstantSized for CMSG_BUYBACK_ITEM {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_BUYBACK_ITEM {
    fn maximum_possible_size() -> usize {
        8 // guid: Guid
        + BuybackSlot::size() // slot: BuybackSlot
    }
}

#[derive(Debug)]
pub enum CMSG_BUYBACK_ITEMError {
    Io(std::io::Error),
    BuybackSlot(BuybackSlotError),
}

impl std::error::Error for CMSG_BUYBACK_ITEMError {}
impl std::fmt::Display for CMSG_BUYBACK_ITEMError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::BuybackSlot(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_BUYBACK_ITEMError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<BuybackSlotError> for CMSG_BUYBACK_ITEMError {
    fn from(e: BuybackSlotError) -> Self {
        Self::BuybackSlot(e)
    }
}

