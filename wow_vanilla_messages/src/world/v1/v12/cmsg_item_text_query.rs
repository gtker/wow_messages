use std::convert::{TryFrom, TryInto};
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_ITEM_TEXT_QUERY {
    pub item_text_id: u32,
    pub mail_id: u32,
    pub unknown1: u32,
}

impl ClientMessageWrite for CMSG_ITEM_TEXT_QUERY {}

impl MessageBody for CMSG_ITEM_TEXT_QUERY {
    const OPCODE: u16 = 0x0243;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // item_text_id: u32
        let item_text_id = crate::util::read_u32_le(r)?;

        // mail_id: u32
        let mail_id = crate::util::read_u32_le(r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        Ok(Self {
            item_text_id,
            mail_id,
            unknown1,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // item_text_id: u32
        w.write_all(&self.item_text_id.to_le_bytes())?;

        // mail_id: u32
        w.write_all(&self.mail_id.to_le_bytes())?;

        // unknown1: u32
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
            // item_text_id: u32
            let item_text_id = crate::util::tokio_read_u32_le(r).await?;

            // mail_id: u32
            let mail_id = crate::util::tokio_read_u32_le(r).await?;

            // unknown1: u32
            let unknown1 = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                item_text_id,
                mail_id,
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
            // item_text_id: u32
            w.write_all(&self.item_text_id.to_le_bytes()).await?;

            // mail_id: u32
            w.write_all(&self.mail_id.to_le_bytes()).await?;

            // unknown1: u32
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
            // item_text_id: u32
            let item_text_id = crate::util::astd_read_u32_le(r).await?;

            // mail_id: u32
            let mail_id = crate::util::astd_read_u32_le(r).await?;

            // unknown1: u32
            let unknown1 = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                item_text_id,
                mail_id,
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
            // item_text_id: u32
            w.write_all(&self.item_text_id.to_le_bytes()).await?;

            // mail_id: u32
            w.write_all(&self.mail_id.to_le_bytes()).await?;

            // unknown1: u32
            w.write_all(&self.unknown1.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl ConstantSized for CMSG_ITEM_TEXT_QUERY {}

impl MaximumPossibleSized for CMSG_ITEM_TEXT_QUERY {
    fn maximum_possible_size() -> usize {
        0
        + 4 // item_text_id: u32
        + 4 // mail_id: u32
        + 4 // unknown1: u32
    }
}

