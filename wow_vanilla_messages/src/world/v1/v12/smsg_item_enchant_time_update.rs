use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ServerMessage, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_ITEM_ENCHANT_TIME_UPDATE {
    pub item_guid: Guid,
    pub slot: u32,
    pub duration: u32,
    pub player_guid: Guid,
}

impl ServerMessage for SMSG_ITEM_ENCHANT_TIME_UPDATE {}

impl SMSG_ITEM_ENCHANT_TIME_UPDATE {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 24], std::io::Error> {
        let mut array_w = [0u8; 24];
        let mut w = array_w.as_mut_slice();
        // item_guid: Guid
        w.write_all(&self.item_guid.guid().to_le_bytes())?;

        // slot: u32
        w.write_all(&self.slot.to_le_bytes())?;

        // duration: u32
        w.write_all(&self.duration.to_le_bytes())?;

        // player_guid: Guid
        w.write_all(&self.player_guid.guid().to_le_bytes())?;

        Ok(array_w)
    }
}

impl MessageBody for SMSG_ITEM_ENCHANT_TIME_UPDATE {
    const OPCODE: u16 = 0x01eb;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        24
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

