use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{SellItemResult, SellItemResultError};
use crate::{WorldServerMessageWrite, WorldMessageBody};
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
pub struct SMSG_SELL_ITEM {
    pub guid: Guid,
    pub item: Guid,
    pub result: SellItemResult,
}

impl WorldServerMessageWrite for SMSG_SELL_ITEM {
    const OPCODE: u16 = 0x1a1;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (Self::size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (Self::size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_SELL_ITEM {
    type Error = SMSG_SELL_ITEMError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // item: Guid
        let item = Guid::read(r)?;

        // result: SellItemResult
        let result = SellItemResult::read(r)?;

        Ok(Self {
            guid,
            item,
            result,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // item: Guid
        self.item.write(w)?;

        // result: SellItemResult
        self.result.write(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_SELL_ITEM {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_SELL_ITEM {
    fn maximum_possible_size() -> usize {
        8 // guid: Guid
        + 8 // item: Guid
        + SellItemResult::size() // result: SellItemResult
    }
}

#[derive(Debug)]
pub enum SMSG_SELL_ITEMError {
    Io(std::io::Error),
    SellItemResult(SellItemResultError),
}

impl std::error::Error for SMSG_SELL_ITEMError {}
impl std::fmt::Display for SMSG_SELL_ITEMError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::SellItemResult(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_SELL_ITEMError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<SellItemResultError> for SMSG_SELL_ITEMError {
    fn from(e: SellItemResultError) -> Self {
        Self::SellItemResult(e)
    }
}

