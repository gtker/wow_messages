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
pub struct SMSG_INSTANCE_SAVE_CREATED {
    pub unknown: u32,
}

impl SMSG_INSTANCE_SAVE_CREATED {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 4], std::io::Error> {
        let mut array_w = [0u8; 4];
        let mut w = array_w.as_mut_slice();
        // unknown: u32
        w.write_all(&self.unknown.to_le_bytes())?;

        Ok(array_w)
    }
}

impl ServerMessage for SMSG_INSTANCE_SAVE_CREATED {
    fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(4);
        // unknown: u32
        w.write_all(&self.unknown.to_le_bytes())?;

        Ok(w)
    }
    const OPCODE: u16 = 0x02cb;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        4
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // unknown: u32
        let unknown = crate::util::read_u32_le(r)?;

        Ok(Self {
            unknown,
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
            // unknown: u32
            let unknown = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                unknown,
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
            // unknown: u32
            let unknown = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                unknown,
            })
        })
    }

}

