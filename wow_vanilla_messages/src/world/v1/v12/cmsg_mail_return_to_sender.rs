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
pub struct CMSG_MAIL_RETURN_TO_SENDER {
    pub mailbox_id: Guid,
    pub mail_id: u32,
}

impl ClientMessageWrite for CMSG_MAIL_RETURN_TO_SENDER {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for CMSG_MAIL_RETURN_TO_SENDER {
    const OPCODE: u16 = 0x0248;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // mailbox_id: Guid
        let mailbox_id = Guid::read(r)?;

        // mail_id: u32
        let mail_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            mailbox_id,
            mail_id,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // mailbox_id: Guid
        self.mailbox_id.write(w)?;

        // mail_id: u32
        w.write_all(&self.mail_id.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // mailbox_id: Guid
        let mailbox_id = Guid::tokio_read(r).await?;

        // mail_id: u32
        let mail_id = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            mailbox_id,
            mail_id,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // mailbox_id: Guid
        self.mailbox_id.tokio_write(w).await?;

        // mail_id: u32
        w.write_all(&self.mail_id.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // mailbox_id: Guid
        let mailbox_id = Guid::astd_read(r).await?;

        // mail_id: u32
        let mail_id = crate::util::astd_read_u32_le(r).await?;

        Ok(Self {
            mailbox_id,
            mail_id,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // mailbox_id: Guid
        self.mailbox_id.astd_write(w).await?;

        // mail_id: u32
        w.write_all(&self.mail_id.to_le_bytes()).await?;

        Ok(())
    }
}

impl ConstantSized for CMSG_MAIL_RETURN_TO_SENDER {}

impl MaximumPossibleSized for CMSG_MAIL_RETURN_TO_SENDER {
    fn maximum_possible_size() -> usize {
        8 // mailbox_id: Guid
        + 4 // mail_id: u32
    }
}

