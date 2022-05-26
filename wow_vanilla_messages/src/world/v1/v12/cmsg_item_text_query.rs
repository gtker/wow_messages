use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_ITEM_TEXT_QUERY {
    pub item_text_id: u32,
    pub mail_id: u32,
    pub unknown1: u32,
}

impl CMSG_ITEM_TEXT_QUERY {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 12], std::io::Error> {
        let mut array_w = [0u8; 12];
        let mut w = array_w.as_mut_slice();
        // item_text_id: u32
        w.write_all(&self.item_text_id.to_le_bytes())?;

        // mail_id: u32
        w.write_all(&self.mail_id.to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(array_w)
    }
}

impl ClientMessage for CMSG_ITEM_TEXT_QUERY {
    fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(12);
        // item_text_id: u32
        w.write_all(&self.item_text_id.to_le_bytes())?;

        // mail_id: u32
        w.write_all(&self.mail_id.to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(w)
    }
    const OPCODE: u16 = 0x0243;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        12
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

}

