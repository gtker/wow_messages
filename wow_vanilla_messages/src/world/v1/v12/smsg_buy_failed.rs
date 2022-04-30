use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{BuyResult, BuyResultError};
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
pub struct SMSG_BUY_FAILED {
    pub guid: Guid,
    pub item_id: u32,
    pub result: BuyResult,
}

impl ServerMessageWrite for SMSG_BUY_FAILED {}

impl MessageBody for SMSG_BUY_FAILED {
    const OPCODE: u16 = 0x01a5;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_BUY_FAILEDError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // item_id: u32
        let item_id = crate::util::read_u32_le(r)?;

        // result: BuyResult
        let result = BuyResult::read(r)?;

        Ok(Self {
            guid,
            item_id,
            result,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // item_id: u32
        w.write_all(&self.item_id.to_le_bytes())?;

        // result: BuyResult
        self.result.write(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_BUY_FAILED {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_BUY_FAILED {
    fn maximum_possible_size() -> usize {
        8 // guid: Guid
        + 4 // item_id: u32
        + BuyResult::size() // result: BuyResult
    }
}

#[derive(Debug)]
pub enum SMSG_BUY_FAILEDError {
    Io(std::io::Error),
    BuyResult(BuyResultError),
}

impl std::error::Error for SMSG_BUY_FAILEDError {}
impl std::fmt::Display for SMSG_BUY_FAILEDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::BuyResult(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_BUY_FAILEDError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<BuyResultError> for SMSG_BUY_FAILEDError {
    fn from(e: BuyResultError) -> Self {
        Self::BuyResult(e)
    }
}

