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
pub struct CMSG_QUESTLOG_SWAP_QUEST {
    pub slot1: u8,
    pub slot2: u8,
}

impl CMSG_QUESTLOG_SWAP_QUEST {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 2], std::io::Error> {
        let mut array_w = [0u8; 2];
        let mut w = array_w.as_mut_slice();
        // slot1: u8
        w.write_all(&self.slot1.to_le_bytes())?;

        // slot2: u8
        w.write_all(&self.slot2.to_le_bytes())?;

        Ok(array_w)
    }
}

impl ClientMessage for CMSG_QUESTLOG_SWAP_QUEST {
    fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(2);
        // slot1: u8
        w.write_all(&self.slot1.to_le_bytes())?;

        // slot2: u8
        w.write_all(&self.slot2.to_le_bytes())?;

        Ok(w)
    }
    const OPCODE: u16 = 0x0193;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        2
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // slot1: u8
        let slot1 = crate::util::read_u8_le(r)?;

        // slot2: u8
        let slot2 = crate::util::read_u8_le(r)?;

        Ok(Self {
            slot1,
            slot2,
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
            // slot1: u8
            let slot1 = crate::util::tokio_read_u8_le(r).await?;

            // slot2: u8
            let slot2 = crate::util::tokio_read_u8_le(r).await?;

            Ok(Self {
                slot1,
                slot2,
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
            // slot1: u8
            let slot1 = crate::util::astd_read_u8_le(r).await?;

            // slot2: u8
            let slot2 = crate::util::astd_read_u8_le(r).await?;

            Ok(Self {
                slot1,
                slot2,
            })
        })
    }

}

