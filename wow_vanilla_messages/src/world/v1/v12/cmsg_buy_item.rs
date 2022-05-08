use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_BUY_ITEM {
    pub vendor_guid: Guid,
    pub item_id: u32,
    pub amount: u8,
    pub unknown1: u8,
}

impl ClientMessageWrite for CMSG_BUY_ITEM {}

impl MessageBody for CMSG_BUY_ITEM {
    const OPCODE: u16 = 0x01a2;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // vendor_guid: Guid
        let vendor_guid = Guid::read(r)?;

        // item_id: u32
        let item_id = crate::util::read_u32_le(r)?;

        // amount: u8
        let amount = crate::util::read_u8_le(r)?;

        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(r)?;

        Ok(Self {
            vendor_guid,
            item_id,
            amount,
            unknown1,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // vendor_guid: Guid
        self.vendor_guid.write(w)?;

        // item_id: u32
        w.write_all(&self.item_id.to_le_bytes())?;

        // amount: u8
        w.write_all(&self.amount.to_le_bytes())?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
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

            // item_id: u32
            let item_id = crate::util::tokio_read_u32_le(r).await?;

            // amount: u8
            let amount = crate::util::tokio_read_u8_le(r).await?;

            // unknown1: u8
            let unknown1 = crate::util::tokio_read_u8_le(r).await?;

            Ok(Self {
                vendor_guid,
                item_id,
                amount,
                unknown1,
            })
        })
    }

    #[cfg(feature = "async_tokio")]
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

            // item_id: u32
            w.write_all(&self.item_id.to_le_bytes()).await?;

            // amount: u8
            w.write_all(&self.amount.to_le_bytes()).await?;

            // unknown1: u8
            w.write_all(&self.unknown1.to_le_bytes()).await?;

            Ok(())
        })
    }

    #[cfg(feature = "async_std")]
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

            // item_id: u32
            let item_id = crate::util::astd_read_u32_le(r).await?;

            // amount: u8
            let amount = crate::util::astd_read_u8_le(r).await?;

            // unknown1: u8
            let unknown1 = crate::util::astd_read_u8_le(r).await?;

            Ok(Self {
                vendor_guid,
                item_id,
                amount,
                unknown1,
            })
        })
    }

    #[cfg(feature = "async_std")]
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

            // item_id: u32
            w.write_all(&self.item_id.to_le_bytes()).await?;

            // amount: u8
            w.write_all(&self.amount.to_le_bytes()).await?;

            // unknown1: u8
            w.write_all(&self.unknown1.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl ConstantSized for CMSG_BUY_ITEM {}

impl MaximumPossibleSized for CMSG_BUY_ITEM {
    fn maximum_possible_size() -> usize {
        0
        + 8 // vendor_guid: Guid
        + 4 // item_id: u32
        + 1 // amount: u8
        + 1 // unknown1: u8
    }
}

