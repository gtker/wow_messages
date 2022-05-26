use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_GUILD_MOTD {
    pub message_of_the_day: String,
}

impl CMSG_GUILD_MOTD {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // message_of_the_day: CString
        w.write_all(self.message_of_the_day.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(w)
    }
}

impl ClientMessage for CMSG_GUILD_MOTD {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // message_of_the_day: CString
        w.write_all(self.message_of_the_day.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    const OPCODE: u16 = 0x0091;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = crate::errors::ParseError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // message_of_the_day: CString
        let message_of_the_day = crate::util::read_c_string_to_vec(r)?;
        let message_of_the_day = String::from_utf8(message_of_the_day)?;

        Ok(Self {
            message_of_the_day,
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
            // message_of_the_day: CString
            let message_of_the_day = crate::util::tokio_read_c_string_to_vec(r).await?;
            let message_of_the_day = String::from_utf8(message_of_the_day)?;

            Ok(Self {
                message_of_the_day,
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
            // message_of_the_day: CString
            let message_of_the_day = crate::util::astd_read_c_string_to_vec(r).await?;
            let message_of_the_day = String::from_utf8(message_of_the_day)?;

            Ok(Self {
                message_of_the_day,
            })
        })
    }

}

impl CMSG_GUILD_MOTD {
    pub fn size(&self) -> usize {
        0
        + self.message_of_the_day.len() + 1 // message_of_the_day: CString
    }
}

