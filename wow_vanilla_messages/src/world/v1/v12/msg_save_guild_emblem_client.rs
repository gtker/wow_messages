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
pub struct MSG_SAVE_GUILD_EMBLEM_Client {
    pub vendor: Guid,
    pub emblem_style: u32,
    pub emblem_color: u32,
    pub border_style: u32,
    pub border_color: u32,
    pub background_color: u32,
}

impl ClientMessageWrite for MSG_SAVE_GUILD_EMBLEM_Client {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for MSG_SAVE_GUILD_EMBLEM_Client {
    const OPCODE: u16 = 0x01f1;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

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

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // vendor: Guid
        self.vendor.write(w)?;

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

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
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
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // vendor: Guid
        self.vendor.tokio_write(w).await?;

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
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
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
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // vendor: Guid
        self.vendor.astd_write(w).await?;

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
    }

}

impl ConstantSized for MSG_SAVE_GUILD_EMBLEM_Client {}

impl MaximumPossibleSized for MSG_SAVE_GUILD_EMBLEM_Client {
    fn maximum_possible_size() -> usize {
        8 // vendor: Guid
        + 4 // emblem_style: u32
        + 4 // emblem_color: u32
        + 4 // border_style: u32
        + 4 // border_color: u32
        + 4 // background_color: u32
    }
}

