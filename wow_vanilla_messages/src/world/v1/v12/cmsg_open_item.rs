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
pub struct CMSG_OPEN_ITEM {
    pub bag_index: u8,
    pub slot: u8,
}

impl CMSG_OPEN_ITEM {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 2], std::io::Error> {
        let mut array_w = [0u8; 2];
        let mut w = array_w.as_mut_slice();
        // bag_index: u8
        w.write_all(&self.bag_index.to_le_bytes())?;

        // slot: u8
        w.write_all(&self.slot.to_le_bytes())?;

        Ok(array_w)
    }
}

impl ClientMessage for CMSG_OPEN_ITEM {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // bag_index: u8
        w.write_all(&self.bag_index.to_le_bytes())?;

        // slot: u8
        w.write_all(&self.slot.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x00ac;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        2
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // bag_index: u8
        let bag_index = crate::util::read_u8_le(r)?;

        // slot: u8
        let slot = crate::util::read_u8_le(r)?;

        Ok(Self {
            bag_index,
            slot,
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
            // bag_index: u8
            let bag_index = crate::util::tokio_read_u8_le(r).await?;

            // slot: u8
            let slot = crate::util::tokio_read_u8_le(r).await?;

            Ok(Self {
                bag_index,
                slot,
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
            // bag_index: u8
            let bag_index = crate::util::astd_read_u8_le(r).await?;

            // slot: u8
            let slot = crate::util::astd_read_u8_le(r).await?;

            Ok(Self {
                bag_index,
                slot,
            })
        })
    }

}

