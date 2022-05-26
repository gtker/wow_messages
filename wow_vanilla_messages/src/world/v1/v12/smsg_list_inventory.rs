use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::ListInventoryItem;
use crate::{ServerMessage, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_LIST_INVENTORY {
    pub vendor: Guid,
    pub items: Vec<ListInventoryItem>,
}

impl ServerMessage for SMSG_LIST_INVENTORY {}

impl SMSG_LIST_INVENTORY {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // vendor: Guid
        w.write_all(&self.vendor.guid().to_le_bytes())?;

        // amount_of_items: u8
        w.write_all(&(self.items.len() as u8).to_le_bytes())?;

        // items: ListInventoryItem[amount_of_items]
        for i in self.items.iter() {
            w.write_all(&(i.as_bytes()?))?;
        }

        Ok(w)
    }
}

impl MessageBody for SMSG_LIST_INVENTORY {
    const OPCODE: u16 = 0x019f;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // vendor: Guid
        let vendor = Guid::read(r)?;

        // amount_of_items: u8
        let amount_of_items = crate::util::read_u8_le(r)?;

        // items: ListInventoryItem[amount_of_items]
        let mut items = Vec::with_capacity(amount_of_items as usize);
        for i in 0..amount_of_items {
            items.push(ListInventoryItem::read(r)?);
        }

        Ok(Self {
            vendor,
            items,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let inner = self.as_bytes()?;
        w.write_all(&inner)
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
            // vendor: Guid
            let vendor = Guid::tokio_read(r).await?;

            // amount_of_items: u8
            let amount_of_items = crate::util::tokio_read_u8_le(r).await?;

            // items: ListInventoryItem[amount_of_items]
            let mut items = Vec::with_capacity(amount_of_items as usize);
            for i in 0..amount_of_items {
                items.push(ListInventoryItem::tokio_read(r).await?);
            }

            Ok(Self {
                vendor,
                items,
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
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let inner = self.as_bytes()?;
            w.write_all(&inner).await
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
            // vendor: Guid
            let vendor = Guid::astd_read(r).await?;

            // amount_of_items: u8
            let amount_of_items = crate::util::astd_read_u8_le(r).await?;

            // items: ListInventoryItem[amount_of_items]
            let mut items = Vec::with_capacity(amount_of_items as usize);
            for i in 0..amount_of_items {
                items.push(ListInventoryItem::astd_read(r).await?);
            }

            Ok(Self {
                vendor,
                items,
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
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let inner = self.as_bytes()?;
            w.write_all(&inner).await
        })
    }

}

impl SMSG_LIST_INVENTORY {
    pub fn size(&self) -> usize {
        0
        + 8 // vendor: Guid
        + 1 // amount_of_items: u8
        + self.items.len() * 28 // items: ListInventoryItem[amount_of_items]
    }
}

