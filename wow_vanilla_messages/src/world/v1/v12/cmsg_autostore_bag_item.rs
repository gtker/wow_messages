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
pub struct CMSG_AUTOSTORE_BAG_ITEM {
    pub source_bag: u8,
    pub source_slot: u8,
    pub destination_bag: u8,
}

impl CMSG_AUTOSTORE_BAG_ITEM {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 3], std::io::Error> {
        let mut array_w = [0u8; 3];
        let mut w = array_w.as_mut_slice();
        // source_bag: u8
        w.write_all(&self.source_bag.to_le_bytes())?;

        // source_slot: u8
        w.write_all(&self.source_slot.to_le_bytes())?;

        // destination_bag: u8
        w.write_all(&self.destination_bag.to_le_bytes())?;

        Ok(array_w)
    }
}

impl ClientMessage for CMSG_AUTOSTORE_BAG_ITEM {
    fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(3);
        // source_bag: u8
        w.write_all(&self.source_bag.to_le_bytes())?;

        // source_slot: u8
        w.write_all(&self.source_slot.to_le_bytes())?;

        // destination_bag: u8
        w.write_all(&self.destination_bag.to_le_bytes())?;

        Ok(w)
    }
    const OPCODE: u16 = 0x010b;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        3
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // source_bag: u8
        let source_bag = crate::util::read_u8_le(r)?;

        // source_slot: u8
        let source_slot = crate::util::read_u8_le(r)?;

        // destination_bag: u8
        let destination_bag = crate::util::read_u8_le(r)?;

        Ok(Self {
            source_bag,
            source_slot,
            destination_bag,
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
            // source_bag: u8
            let source_bag = crate::util::tokio_read_u8_le(r).await?;

            // source_slot: u8
            let source_slot = crate::util::tokio_read_u8_le(r).await?;

            // destination_bag: u8
            let destination_bag = crate::util::tokio_read_u8_le(r).await?;

            Ok(Self {
                source_bag,
                source_slot,
                destination_bag,
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
            // source_bag: u8
            let source_bag = crate::util::astd_read_u8_le(r).await?;

            // source_slot: u8
            let source_slot = crate::util::astd_read_u8_le(r).await?;

            // destination_bag: u8
            let destination_bag = crate::util::astd_read_u8_le(r).await?;

            Ok(Self {
                source_bag,
                source_slot,
                destination_bag,
            })
        })
    }

}

