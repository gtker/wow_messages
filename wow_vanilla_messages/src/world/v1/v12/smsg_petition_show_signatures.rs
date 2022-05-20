use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_PETITION_SHOW_SIGNATURES {
    pub item_guid: Guid,
    pub owner_guid: Guid,
    pub petition_guid: Guid,
    pub amount_of_signatures: u8,
}

impl ServerMessageWrite for SMSG_PETITION_SHOW_SIGNATURES {}

impl SMSG_PETITION_SHOW_SIGNATURES {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 25], std::io::Error> {
        let mut array_w = [0u8; 25];
        let mut w = array_w.as_mut_slice();
        // item_guid: Guid
        w.write_all(&self.item_guid.guid().to_le_bytes())?;

        // owner_guid: Guid
        w.write_all(&self.owner_guid.guid().to_le_bytes())?;

        // petition_guid: Guid
        w.write_all(&self.petition_guid.guid().to_le_bytes())?;

        // amount_of_signatures: u8
        w.write_all(&self.amount_of_signatures.to_le_bytes())?;

        Ok(array_w)
    }
}

impl MessageBody for SMSG_PETITION_SHOW_SIGNATURES {
    const OPCODE: u16 = 0x01bf;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // item_guid: Guid
        let item_guid = Guid::read(r)?;

        // owner_guid: Guid
        let owner_guid = Guid::read(r)?;

        // petition_guid: Guid
        let petition_guid = Guid::read(r)?;

        // amount_of_signatures: u8
        let amount_of_signatures = crate::util::read_u8_le(r)?;

        Ok(Self {
            item_guid,
            owner_guid,
            petition_guid,
            amount_of_signatures,
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

            // owner_guid: Guid
            let owner_guid = Guid::tokio_read(r).await?;

            // petition_guid: Guid
            let petition_guid = Guid::tokio_read(r).await?;

            // amount_of_signatures: u8
            let amount_of_signatures = crate::util::tokio_read_u8_le(r).await?;

            Ok(Self {
                item_guid,
                owner_guid,
                petition_guid,
                amount_of_signatures,
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

            // owner_guid: Guid
            let owner_guid = Guid::astd_read(r).await?;

            // petition_guid: Guid
            let petition_guid = Guid::astd_read(r).await?;

            // amount_of_signatures: u8
            let amount_of_signatures = crate::util::astd_read_u8_le(r).await?;

            Ok(Self {
                item_guid,
                owner_guid,
                petition_guid,
                amount_of_signatures,
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

impl SMSG_PETITION_SHOW_SIGNATURES {
    pub(crate) fn size() -> usize {
        0
        + 8 // item_guid: Guid
        + 8 // owner_guid: Guid
        + 8 // petition_guid: Guid
        + 1 // amount_of_signatures: u8
    }
}

