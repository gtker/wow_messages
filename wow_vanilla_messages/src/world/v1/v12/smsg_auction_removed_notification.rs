use std::convert::{TryFrom, TryInto};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_AUCTION_REMOVED_NOTIFICATION {
    pub item_id: u32,
    pub item_template: u32,
    pub random_property_id: u32,
}

impl ServerMessageWrite for SMSG_AUCTION_REMOVED_NOTIFICATION {}

impl MessageBody for SMSG_AUCTION_REMOVED_NOTIFICATION {
    const OPCODE: u16 = 0x028d;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // item_id: u32
        let item_id = crate::util::read_u32_le(r)?;

        // item_template: u32
        let item_template = crate::util::read_u32_le(r)?;

        // random_property_id: u32
        let random_property_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            item_id,
            item_template,
            random_property_id,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // item_id: u32
        w.write_all(&self.item_id.to_le_bytes())?;

        // item_template: u32
        w.write_all(&self.item_template.to_le_bytes())?;

        // random_property_id: u32
        w.write_all(&self.random_property_id.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "tokio")]
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
            // item_id: u32
            let item_id = crate::util::tokio_read_u32_le(r).await?;

            // item_template: u32
            let item_template = crate::util::tokio_read_u32_le(r).await?;

            // random_property_id: u32
            let random_property_id = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                item_id,
                item_template,
                random_property_id,
            })
        })
    }

    #[cfg(feature = "tokio")]
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
            // item_id: u32
            w.write_all(&self.item_id.to_le_bytes()).await?;

            // item_template: u32
            w.write_all(&self.item_template.to_le_bytes()).await?;

            // random_property_id: u32
            w.write_all(&self.random_property_id.to_le_bytes()).await?;

            Ok(())
        })
    }

    #[cfg(feature = "async-std")]
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
            // item_id: u32
            let item_id = crate::util::astd_read_u32_le(r).await?;

            // item_template: u32
            let item_template = crate::util::astd_read_u32_le(r).await?;

            // random_property_id: u32
            let random_property_id = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                item_id,
                item_template,
                random_property_id,
            })
        })
    }

    #[cfg(feature = "async-std")]
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
            // item_id: u32
            w.write_all(&self.item_id.to_le_bytes()).await?;

            // item_template: u32
            w.write_all(&self.item_template.to_le_bytes()).await?;

            // random_property_id: u32
            w.write_all(&self.random_property_id.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl SMSG_AUCTION_REMOVED_NOTIFICATION {
    pub(crate) fn size() -> usize {
        0
        + 4 // item_id: u32
        + 4 // item_template: u32
        + 4 // random_property_id: u32
    }
}

