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
pub struct CMSG_GUILD_QUERY {
    pub guild_id: u32,
}

impl CMSG_GUILD_QUERY {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 4], std::io::Error> {
        let mut array_w = [0u8; 4];
        let mut w = array_w.as_mut_slice();
        // guild_id: u32
        w.write_all(&self.guild_id.to_le_bytes())?;

        Ok(array_w)
    }
}

impl ClientMessage for CMSG_GUILD_QUERY {
    fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(4);
        // guild_id: u32
        w.write_all(&self.guild_id.to_le_bytes())?;

        Ok(w)
    }
    const OPCODE: u16 = 0x0054;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        4
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guild_id: u32
        let guild_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            guild_id,
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
            // guild_id: u32
            let guild_id = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                guild_id,
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
            // guild_id: u32
            let guild_id = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                guild_id,
            })
        })
    }

}

