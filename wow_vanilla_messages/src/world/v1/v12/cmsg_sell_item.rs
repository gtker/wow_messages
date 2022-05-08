use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ClientMessageWrite, MessageBody};
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
pub struct CMSG_SELL_ITEM {
    pub vendor_guid: Guid,
    pub item_guid: Guid,
    pub amount: u8,
}

impl ClientMessageWrite for CMSG_SELL_ITEM {}

impl MessageBody for CMSG_SELL_ITEM {
    const OPCODE: u16 = 0x01a0;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // vendor_guid: Guid
        let vendor_guid = Guid::read(r)?;

        // item_guid: Guid
        let item_guid = Guid::read(r)?;

        // amount: u8
        let amount = crate::util::read_u8_le(r)?;

        Ok(Self {
            vendor_guid,
            item_guid,
            amount,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // vendor_guid: Guid
        self.vendor_guid.write(w)?;

        // item_guid: Guid
        self.item_guid.write(w)?;

        // amount: u8
        w.write_all(&self.amount.to_le_bytes())?;

        Ok(())
    }

    fn tokio_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // vendor_guid: Guid
            let vendor_guid = Guid::tokio_read(r).await?;

            // item_guid: Guid
            let item_guid = Guid::tokio_read(r).await?;

            // amount: u8
            let amount = crate::util::tokio_read_u8_le(r).await?;

            Ok(Self {
                vendor_guid,
                item_guid,
                amount,
            })
        })
    }

    fn tokio_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // vendor_guid: Guid
            self.vendor_guid.tokio_write(w).await?;

            // item_guid: Guid
            self.item_guid.tokio_write(w).await?;

            // amount: u8
            w.write_all(&self.amount.to_le_bytes()).await?;

            Ok(())
        })
    }

    fn astd_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // vendor_guid: Guid
            let vendor_guid = Guid::astd_read(r).await?;

            // item_guid: Guid
            let item_guid = Guid::astd_read(r).await?;

            // amount: u8
            let amount = crate::util::astd_read_u8_le(r).await?;

            Ok(Self {
                vendor_guid,
                item_guid,
                amount,
            })
        })
    }

    fn astd_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // vendor_guid: Guid
            self.vendor_guid.astd_write(w).await?;

            // item_guid: Guid
            self.item_guid.astd_write(w).await?;

            // amount: u8
            w.write_all(&self.amount.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl ConstantSized for CMSG_SELL_ITEM {}

impl MaximumPossibleSized for CMSG_SELL_ITEM {
    fn maximum_possible_size() -> usize {
        0
        + 8 // vendor_guid: Guid
        + 8 // item_guid: Guid
        + 1 // amount: u8
    }
}

