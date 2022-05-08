use std::convert::{TryFrom, TryInto};
use crate::Guid;
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
pub struct SMSG_ITEM_ENCHANT_TIME_UPDATE {
    pub item_guid: Guid,
    pub slot: u32,
    pub duration: u32,
    pub player_guid: Guid,
}

impl ServerMessageWrite for SMSG_ITEM_ENCHANT_TIME_UPDATE {}

impl MessageBody for SMSG_ITEM_ENCHANT_TIME_UPDATE {
    const OPCODE: u16 = 0x01eb;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // item_guid: Guid
        let item_guid = Guid::read(r)?;

        // slot: u32
        let slot = crate::util::read_u32_le(r)?;

        // duration: u32
        let duration = crate::util::read_u32_le(r)?;

        // player_guid: Guid
        let player_guid = Guid::read(r)?;

        Ok(Self {
            item_guid,
            slot,
            duration,
            player_guid,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // item_guid: Guid
        self.item_guid.write(w)?;

        // slot: u32
        w.write_all(&self.slot.to_le_bytes())?;

        // duration: u32
        w.write_all(&self.duration.to_le_bytes())?;

        // player_guid: Guid
        self.player_guid.write(w)?;

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
            // item_guid: Guid
            let item_guid = Guid::tokio_read(r).await?;

            // slot: u32
            let slot = crate::util::tokio_read_u32_le(r).await?;

            // duration: u32
            let duration = crate::util::tokio_read_u32_le(r).await?;

            // player_guid: Guid
            let player_guid = Guid::tokio_read(r).await?;

            Ok(Self {
                item_guid,
                slot,
                duration,
                player_guid,
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
            // item_guid: Guid
            self.item_guid.tokio_write(w).await?;

            // slot: u32
            w.write_all(&self.slot.to_le_bytes()).await?;

            // duration: u32
            w.write_all(&self.duration.to_le_bytes()).await?;

            // player_guid: Guid
            self.player_guid.tokio_write(w).await?;

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
            // item_guid: Guid
            let item_guid = Guid::astd_read(r).await?;

            // slot: u32
            let slot = crate::util::astd_read_u32_le(r).await?;

            // duration: u32
            let duration = crate::util::astd_read_u32_le(r).await?;

            // player_guid: Guid
            let player_guid = Guid::astd_read(r).await?;

            Ok(Self {
                item_guid,
                slot,
                duration,
                player_guid,
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
            // item_guid: Guid
            self.item_guid.astd_write(w).await?;

            // slot: u32
            w.write_all(&self.slot.to_le_bytes()).await?;

            // duration: u32
            w.write_all(&self.duration.to_le_bytes()).await?;

            // player_guid: Guid
            self.player_guid.astd_write(w).await?;

            Ok(())
        })
    }

}

impl ConstantSized for SMSG_ITEM_ENCHANT_TIME_UPDATE {}

impl MaximumPossibleSized for SMSG_ITEM_ENCHANT_TIME_UPDATE {
    fn maximum_possible_size() -> usize {
        0
        + 8 // item_guid: Guid
        + 4 // slot: u32
        + 4 // duration: u32
        + 8 // player_guid: Guid
    }
}

