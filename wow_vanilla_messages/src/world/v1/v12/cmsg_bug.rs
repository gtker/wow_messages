use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_BUG {
    pub suggestion: u32,
    pub content_length: u32,
    pub content: String,
    pub type_length: u32,
    pub bug_type: String,
}

impl CMSG_BUG {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // suggestion: u32
        w.write_all(&self.suggestion.to_le_bytes())?;

        // content_length: u32
        w.write_all(&self.content_length.to_le_bytes())?;

        // content: CString
        w.write_all(self.content.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // type_length: u32
        w.write_all(&self.type_length.to_le_bytes())?;

        // bug_type: CString
        w.write_all(self.bug_type.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(w)
    }
}

impl ClientMessage for CMSG_BUG {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // suggestion: u32
        w.write_all(&self.suggestion.to_le_bytes())?;

        // content_length: u32
        w.write_all(&self.content_length.to_le_bytes())?;

        // content: CString
        w.write_all(self.content.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // type_length: u32
        w.write_all(&self.type_length.to_le_bytes())?;

        // bug_type: CString
        w.write_all(self.bug_type.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    const OPCODE: u16 = 0x01ca;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = crate::errors::ParseError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // suggestion: u32
        let suggestion = crate::util::read_u32_le(r)?;

        // content_length: u32
        let content_length = crate::util::read_u32_le(r)?;

        // content: CString
        let content = crate::util::read_c_string_to_vec(r)?;
        let content = String::from_utf8(content)?;

        // type_length: u32
        let type_length = crate::util::read_u32_le(r)?;

        // bug_type: CString
        let bug_type = crate::util::read_c_string_to_vec(r)?;
        let bug_type = String::from_utf8(bug_type)?;

        Ok(Self {
            suggestion,
            content_length,
            content,
            type_length,
            bug_type,
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
            // suggestion: u32
            let suggestion = crate::util::tokio_read_u32_le(r).await?;

            // content_length: u32
            let content_length = crate::util::tokio_read_u32_le(r).await?;

            // content: CString
            let content = crate::util::tokio_read_c_string_to_vec(r).await?;
            let content = String::from_utf8(content)?;

            // type_length: u32
            let type_length = crate::util::tokio_read_u32_le(r).await?;

            // bug_type: CString
            let bug_type = crate::util::tokio_read_c_string_to_vec(r).await?;
            let bug_type = String::from_utf8(bug_type)?;

            Ok(Self {
                suggestion,
                content_length,
                content,
                type_length,
                bug_type,
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
            // suggestion: u32
            let suggestion = crate::util::astd_read_u32_le(r).await?;

            // content_length: u32
            let content_length = crate::util::astd_read_u32_le(r).await?;

            // content: CString
            let content = crate::util::astd_read_c_string_to_vec(r).await?;
            let content = String::from_utf8(content)?;

            // type_length: u32
            let type_length = crate::util::astd_read_u32_le(r).await?;

            // bug_type: CString
            let bug_type = crate::util::astd_read_c_string_to_vec(r).await?;
            let bug_type = String::from_utf8(bug_type)?;

            Ok(Self {
                suggestion,
                content_length,
                content,
                type_length,
                bug_type,
            })
        })
    }

}

impl CMSG_BUG {
    pub fn size(&self) -> usize {
        0
        + 4 // suggestion: u32
        + 4 // content_length: u32
        + self.content.len() + 1 // content: CString
        + 4 // type_length: u32
        + self.bug_type.len() + 1 // bug_type: CString
    }
}

