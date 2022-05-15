use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::ListInventoryItem;
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_LIST_INVENTORY {
    pub vendor: Guid,
    pub items: Vec<ListInventoryItem>,
}

impl ServerMessageWrite for SMSG_LIST_INVENTORY {}

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
        // vendor: Guid
        self.vendor.write(w)?;

        // amount_of_items: u8
        w.write_all(&(self.items.len() as u8).to_le_bytes())?;

        // items: ListInventoryItem[amount_of_items]
        for i in self.items.iter() {
            i.write(w)?;
        }

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
            // vendor: Guid
            self.vendor.tokio_write(w).await?;

            // amount_of_items: u8
            w.write_all(&(self.items.len() as u8).to_le_bytes()).await?;

            // items: ListInventoryItem[amount_of_items]
            for i in self.items.iter() {
                i.tokio_write(w).await?;
            }

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
            // vendor: Guid
            self.vendor.astd_write(w).await?;

            // amount_of_items: u8
            w.write_all(&(self.items.len() as u8).to_le_bytes()).await?;

            // items: ListInventoryItem[amount_of_items]
            for i in self.items.iter() {
                i.astd_write(w).await?;
            }

            Ok(())
        })
    }

}

impl VariableSized for SMSG_LIST_INVENTORY {
    fn size(&self) -> usize {
        0
        + 8 // vendor: Guid
        + 1 // amount_of_items: u8
        + self.items.iter().fold(0, |acc, x| acc + ListInventoryItem::size()) // items: ListInventoryItem[amount_of_items]
    }
}

impl MaximumPossibleSized for SMSG_LIST_INVENTORY {
    fn maximum_possible_size() -> usize {
        0
        + 8 // vendor: Guid
        + 1 // amount_of_items: u8
        + 7168 // items: ListInventoryItem[amount_of_items]
    }
}

