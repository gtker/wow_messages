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
pub struct CMSG_MAIL_CREATE_TEXT_ITEM {
    pub mailbox_guid: Guid,
    pub mail_id: u32,
    pub mail_template_id: u32,
}

impl ClientMessageWrite for CMSG_MAIL_CREATE_TEXT_ITEM {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for CMSG_MAIL_CREATE_TEXT_ITEM {
    const OPCODE: u16 = 0x024a;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // mailbox_guid: Guid
        let mailbox_guid = Guid::read(r)?;

        // mail_id: u32
        let mail_id = crate::util::read_u32_le(r)?;

        // mail_template_id: u32
        let mail_template_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            mailbox_guid,
            mail_id,
            mail_template_id,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // mailbox_guid: Guid
        self.mailbox_guid.write(w)?;

        // mail_id: u32
        w.write_all(&self.mail_id.to_le_bytes())?;

        // mail_template_id: u32
        w.write_all(&self.mail_template_id.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // mailbox_guid: Guid
        let mailbox_guid = Guid::tokio_read(r).await?;

        // mail_id: u32
        let mail_id = crate::util::tokio_read_u32_le(r).await?;

        // mail_template_id: u32
        let mail_template_id = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            mailbox_guid,
            mail_id,
            mail_template_id,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // mailbox_guid: Guid
        self.mailbox_guid.tokio_write(w).await?;

        // mail_id: u32
        w.write_all(&self.mail_id.to_le_bytes()).await?;

        // mail_template_id: u32
        w.write_all(&self.mail_template_id.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // mailbox_guid: Guid
        let mailbox_guid = Guid::astd_read(r).await?;

        // mail_id: u32
        let mail_id = crate::util::astd_read_u32_le(r).await?;

        // mail_template_id: u32
        let mail_template_id = crate::util::astd_read_u32_le(r).await?;

        Ok(Self {
            mailbox_guid,
            mail_id,
            mail_template_id,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // mailbox_guid: Guid
        self.mailbox_guid.astd_write(w).await?;

        // mail_id: u32
        w.write_all(&self.mail_id.to_le_bytes()).await?;

        // mail_template_id: u32
        w.write_all(&self.mail_template_id.to_le_bytes()).await?;

        Ok(())
    }

}

impl ConstantSized for CMSG_MAIL_CREATE_TEXT_ITEM {}

impl MaximumPossibleSized for CMSG_MAIL_CREATE_TEXT_ITEM {
    fn maximum_possible_size() -> usize {
        0
        + 8 // mailbox_guid: Guid
        + 4 // mail_id: u32
        + 4 // mail_template_id: u32
    }
}

