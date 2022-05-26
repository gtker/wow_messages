use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct MSG_QUERY_NEXT_MAIL_TIME_Server {
    pub unread_mails: f32,
}

impl MSG_QUERY_NEXT_MAIL_TIME_Server {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 4], std::io::Error> {
        let mut array_w = [0u8; 4];
        let mut w = array_w.as_mut_slice();
        // unread_mails: f32
        w.write_all(&self.unread_mails.to_le_bytes())?;

        Ok(array_w)
    }
}

impl ServerMessage for MSG_QUERY_NEXT_MAIL_TIME_Server {
    fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(4);
        // unread_mails: f32
        w.write_all(&self.unread_mails.to_le_bytes())?;

        Ok(w)
    }
    const OPCODE: u16 = 0x0284;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        4
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // unread_mails: f32
        let unread_mails = crate::util::read_f32_le(r)?;
        Ok(Self {
            unread_mails,
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
            // unread_mails: f32
            let unread_mails = crate::util::tokio_read_f32_le(r).await?;
            Ok(Self {
                unread_mails,
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
            // unread_mails: f32
            let unread_mails = crate::util::astd_read_f32_le(r).await?;
            Ok(Self {
                unread_mails,
            })
        })
    }

}

