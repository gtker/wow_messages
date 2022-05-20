use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct MSG_SAVE_GUILD_EMBLEM_Client {
    pub vendor: Guid,
    pub emblem_style: u32,
    pub emblem_color: u32,
    pub border_style: u32,
    pub border_color: u32,
    pub background_color: u32,
}

impl ClientMessageWrite for MSG_SAVE_GUILD_EMBLEM_Client {}

impl MessageBody for MSG_SAVE_GUILD_EMBLEM_Client {
    const OPCODE: u16 = 0x01f1;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // vendor: Guid
        let vendor = Guid::read(r)?;

        // emblem_style: u32
        let emblem_style = crate::util::read_u32_le(r)?;

        // emblem_color: u32
        let emblem_color = crate::util::read_u32_le(r)?;

        // border_style: u32
        let border_style = crate::util::read_u32_le(r)?;

        // border_color: u32
        let border_color = crate::util::read_u32_le(r)?;

        // background_color: u32
        let background_color = crate::util::read_u32_le(r)?;

        Ok(Self {
            vendor,
            emblem_style,
            emblem_color,
            border_style,
            border_color,
            background_color,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // vendor: Guid
        w.write_all(&self.vendor.guid().to_le_bytes())?;

        // emblem_style: u32
        w.write_all(&self.emblem_style.to_le_bytes())?;

        // emblem_color: u32
        w.write_all(&self.emblem_color.to_le_bytes())?;

        // border_style: u32
        w.write_all(&self.border_style.to_le_bytes())?;

        // border_color: u32
        w.write_all(&self.border_color.to_le_bytes())?;

        // background_color: u32
        w.write_all(&self.background_color.to_le_bytes())?;

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
            // vendor: Guid
            let vendor = Guid::tokio_read(r).await?;

            // emblem_style: u32
            let emblem_style = crate::util::tokio_read_u32_le(r).await?;

            // emblem_color: u32
            let emblem_color = crate::util::tokio_read_u32_le(r).await?;

            // border_style: u32
            let border_style = crate::util::tokio_read_u32_le(r).await?;

            // border_color: u32
            let border_color = crate::util::tokio_read_u32_le(r).await?;

            // background_color: u32
            let background_color = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                vendor,
                emblem_style,
                emblem_color,
                border_style,
                border_color,
                background_color,
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
            // vendor: Guid
            w.write_all(&self.vendor.guid().to_le_bytes()).await?;

            // emblem_style: u32
            w.write_all(&self.emblem_style.to_le_bytes()).await?;

            // emblem_color: u32
            w.write_all(&self.emblem_color.to_le_bytes()).await?;

            // border_style: u32
            w.write_all(&self.border_style.to_le_bytes()).await?;

            // border_color: u32
            w.write_all(&self.border_color.to_le_bytes()).await?;

            // background_color: u32
            w.write_all(&self.background_color.to_le_bytes()).await?;

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
            // vendor: Guid
            let vendor = Guid::astd_read(r).await?;

            // emblem_style: u32
            let emblem_style = crate::util::astd_read_u32_le(r).await?;

            // emblem_color: u32
            let emblem_color = crate::util::astd_read_u32_le(r).await?;

            // border_style: u32
            let border_style = crate::util::astd_read_u32_le(r).await?;

            // border_color: u32
            let border_color = crate::util::astd_read_u32_le(r).await?;

            // background_color: u32
            let background_color = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                vendor,
                emblem_style,
                emblem_color,
                border_style,
                border_color,
                background_color,
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
            // vendor: Guid
            w.write_all(&self.vendor.guid().to_le_bytes()).await?;

            // emblem_style: u32
            w.write_all(&self.emblem_style.to_le_bytes()).await?;

            // emblem_color: u32
            w.write_all(&self.emblem_color.to_le_bytes()).await?;

            // border_style: u32
            w.write_all(&self.border_style.to_le_bytes()).await?;

            // border_color: u32
            w.write_all(&self.border_color.to_le_bytes()).await?;

            // background_color: u32
            w.write_all(&self.background_color.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl MSG_SAVE_GUILD_EMBLEM_Client {
    pub(crate) fn size() -> usize {
        0
        + 8 // vendor: Guid
        + 4 // emblem_style: u32
        + 4 // emblem_color: u32
        + 4 // border_style: u32
        + 4 // border_color: u32
        + 4 // background_color: u32
    }
}

