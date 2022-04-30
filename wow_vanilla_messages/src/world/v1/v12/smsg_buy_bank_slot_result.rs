use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{BuyBankSlotResult, BuyBankSlotResultError};
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
pub struct SMSG_BUY_BANK_SLOT_RESULT {
    pub result: BuyBankSlotResult,
}

impl ServerMessageWrite for SMSG_BUY_BANK_SLOT_RESULT {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_BUY_BANK_SLOT_RESULT {
    const OPCODE: u16 = 0x01ba;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_BUY_BANK_SLOT_RESULTError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // result: BuyBankSlotResult
        let result = BuyBankSlotResult::read(r)?;

        Ok(Self {
            result,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // result: BuyBankSlotResult
        self.result.write(w)?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // result: BuyBankSlotResult
        let result = BuyBankSlotResult::tokio_read(r).await?;

        Ok(Self {
            result,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // result: BuyBankSlotResult
        self.result.tokio_write(w).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // result: BuyBankSlotResult
        let result = BuyBankSlotResult::astd_read(r).await?;

        Ok(Self {
            result,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // result: BuyBankSlotResult
        self.result.astd_write(w).await?;

        Ok(())
    }
}

impl ConstantSized for SMSG_BUY_BANK_SLOT_RESULT {}

impl MaximumPossibleSized for SMSG_BUY_BANK_SLOT_RESULT {
    fn maximum_possible_size() -> usize {
        BuyBankSlotResult::size() // result: BuyBankSlotResult
    }
}

#[derive(Debug)]
pub enum SMSG_BUY_BANK_SLOT_RESULTError {
    Io(std::io::Error),
    BuyBankSlotResult(BuyBankSlotResultError),
}

impl std::error::Error for SMSG_BUY_BANK_SLOT_RESULTError {}
impl std::fmt::Display for SMSG_BUY_BANK_SLOT_RESULTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::BuyBankSlotResult(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_BUY_BANK_SLOT_RESULTError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<BuyBankSlotResultError> for SMSG_BUY_BANK_SLOT_RESULTError {
    fn from(e: BuyBankSlotResultError) -> Self {
        Self::BuyBankSlotResult(e)
    }
}

