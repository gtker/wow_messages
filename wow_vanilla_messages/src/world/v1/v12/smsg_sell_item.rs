use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{SellItemResult, SellItemResultError};
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
pub struct SMSG_SELL_ITEM {
    pub guid: Guid,
    pub item: Guid,
    pub result: SellItemResult,
}

impl ServerMessageWrite for SMSG_SELL_ITEM {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_SELL_ITEM {
    const OPCODE: u16 = 0x01a1;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

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

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::tokio_read(r).await?;

        // item: Guid
        let item = Guid::tokio_read(r).await?;

        // result: SellItemResult
        let result = SellItemResult::tokio_read(r).await?;

        Ok(Self {
            guid,
            item,
            result,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.tokio_write(w).await?;

        // item: Guid
        self.item.tokio_write(w).await?;

        // result: SellItemResult
        self.result.tokio_write(w).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::astd_read(r).await?;

        // item: Guid
        let item = Guid::astd_read(r).await?;

        // result: SellItemResult
        let result = SellItemResult::astd_read(r).await?;

        Ok(Self {
            guid,
            item,
            result,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.astd_write(w).await?;

        // item: Guid
        self.item.astd_write(w).await?;

        // result: SellItemResult
        self.result.astd_write(w).await?;

        Ok(())
    }

}

impl ConstantSized for SMSG_SELL_ITEM {}

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

