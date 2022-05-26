use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_PAGE_TEXT_QUERY_RESPONSE {
    pub page_id: u32,
    pub text: String,
    pub next_page_id: u32,
}

impl SMSG_PAGE_TEXT_QUERY_RESPONSE {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // page_id: u32
        w.write_all(&self.page_id.to_le_bytes())?;

        // text: CString
        w.write_all(self.text.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // next_page_id: u32
        w.write_all(&self.next_page_id.to_le_bytes())?;

        Ok(w)
    }
}

impl ServerMessage for SMSG_PAGE_TEXT_QUERY_RESPONSE {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // page_id: u32
        w.write_all(&self.page_id.to_le_bytes())?;

        // text: CString
        w.write_all(self.text.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // next_page_id: u32
        w.write_all(&self.next_page_id.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x005b;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = crate::errors::ParseError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // page_id: u32
        let page_id = crate::util::read_u32_le(r)?;

        // text: CString
        let text = crate::util::read_c_string_to_vec(r)?;
        let text = String::from_utf8(text)?;

        // next_page_id: u32
        let next_page_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            page_id,
            text,
            next_page_id,
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
            // page_id: u32
            let page_id = crate::util::tokio_read_u32_le(r).await?;

            // text: CString
            let text = crate::util::tokio_read_c_string_to_vec(r).await?;
            let text = String::from_utf8(text)?;

            // next_page_id: u32
            let next_page_id = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                page_id,
                text,
                next_page_id,
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
            // page_id: u32
            let page_id = crate::util::astd_read_u32_le(r).await?;

            // text: CString
            let text = crate::util::astd_read_c_string_to_vec(r).await?;
            let text = String::from_utf8(text)?;

            // next_page_id: u32
            let next_page_id = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                page_id,
                text,
                next_page_id,
            })
        })
    }

}

impl SMSG_PAGE_TEXT_QUERY_RESPONSE {
    pub fn size(&self) -> usize {
        0
        + 4 // page_id: u32
        + self.text.len() + 1 // text: CString
        + 4 // next_page_id: u32
    }
}

