use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_SELL_ITEM {
    pub vendor_guid: Guid,
    pub item_guid: Guid,
    pub amount: u8,
}

impl CMSG_SELL_ITEM {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 17], std::io::Error> {
        let mut array_w = [0u8; 17];
        let mut w = array_w.as_mut_slice();
        // vendor_guid: Guid
        w.write_all(&self.vendor_guid.guid().to_le_bytes())?;

        // item_guid: Guid
        w.write_all(&self.item_guid.guid().to_le_bytes())?;

        // amount: u8
        w.write_all(&self.amount.to_le_bytes())?;

        Ok(array_w)
    }
}

impl ClientMessage for CMSG_SELL_ITEM {
    fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(17);
        // vendor_guid: Guid
        w.write_all(&self.vendor_guid.guid().to_le_bytes())?;

        // item_guid: Guid
        w.write_all(&self.item_guid.guid().to_le_bytes())?;

        // amount: u8
        w.write_all(&self.amount.to_le_bytes())?;

        Ok(w)
    }
    const OPCODE: u16 = 0x01a0;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        17
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // vendor_guid: Guid
        let vendor_guid = Guid::read(r)?;

        // item_guid: Guid
        let item_guid = Guid::read(r)?;

        // amount: u8
        let amount = crate::util::read_u8_le(r)?;

        Ok(Self {
            vendor_guid,
            item_guid,
            amount,
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
            // vendor_guid: Guid
            let vendor_guid = Guid::tokio_read(r).await?;

            // item_guid: Guid
            let item_guid = Guid::tokio_read(r).await?;

            // amount: u8
            let amount = crate::util::tokio_read_u8_le(r).await?;

            Ok(Self {
                vendor_guid,
                item_guid,
                amount,
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
            // vendor_guid: Guid
            let vendor_guid = Guid::astd_read(r).await?;

            // item_guid: Guid
            let item_guid = Guid::astd_read(r).await?;

            // amount: u8
            let amount = crate::util::astd_read_u8_le(r).await?;

            Ok(Self {
                vendor_guid,
                item_guid,
                amount,
            })
        })
    }

}

