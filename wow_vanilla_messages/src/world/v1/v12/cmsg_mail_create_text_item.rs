use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_MAIL_CREATE_TEXT_ITEM {
    pub mailbox_guid: Guid,
    pub mail_id: u32,
    pub mail_template_id: u32,
}

impl CMSG_MAIL_CREATE_TEXT_ITEM {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 16], std::io::Error> {
        let mut array_w = [0u8; 16];
        let mut w = array_w.as_mut_slice();
        // mailbox_guid: Guid
        w.write_all(&self.mailbox_guid.guid().to_le_bytes())?;

        // mail_id: u32
        w.write_all(&self.mail_id.to_le_bytes())?;

        // mail_template_id: u32
        w.write_all(&self.mail_template_id.to_le_bytes())?;

        Ok(array_w)
    }
}

impl ClientMessage for CMSG_MAIL_CREATE_TEXT_ITEM {
    fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(16);
        // mailbox_guid: Guid
        w.write_all(&self.mailbox_guid.guid().to_le_bytes())?;

        // mail_id: u32
        w.write_all(&self.mail_id.to_le_bytes())?;

        // mail_template_id: u32
        w.write_all(&self.mail_template_id.to_le_bytes())?;

        Ok(w)
    }
    const OPCODE: u16 = 0x024a;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        16
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
        })
    }

}

