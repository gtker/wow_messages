use std::convert::{TryFrom, TryInto};
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_AUTOBANK_ITEM {
    pub bag_index: u8,
    pub slot_index: u8,
}

impl ClientMessageWrite for CMSG_AUTOBANK_ITEM {}

impl MessageBody for CMSG_AUTOBANK_ITEM {
    const OPCODE: u16 = 0x0283;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // bag_index: u8
        let bag_index = crate::util::read_u8_le(r)?;

        // slot_index: u8
        let slot_index = crate::util::read_u8_le(r)?;

        Ok(Self {
            bag_index,
            slot_index,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // bag_index: u8
        w.write_all(&self.bag_index.to_le_bytes())?;

        // slot_index: u8
        w.write_all(&self.slot_index.to_le_bytes())?;

        Ok(())
    }

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

            // slot_index: u8
            let slot_index = crate::util::tokio_read_u8_le(r).await?;

            Ok(Self {
                bag_index,
                slot_index,
            })
        })
    }

    fn tokio_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // bag_index: u8
            w.write_all(&self.bag_index.to_le_bytes()).await?;

            // slot_index: u8
            w.write_all(&self.slot_index.to_le_bytes()).await?;

            Ok(())
        })
    }

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

            // slot_index: u8
            let slot_index = crate::util::astd_read_u8_le(r).await?;

            Ok(Self {
                bag_index,
                slot_index,
            })
        })
    }

    fn astd_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // bag_index: u8
            w.write_all(&self.bag_index.to_le_bytes()).await?;

            // slot_index: u8
            w.write_all(&self.slot_index.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl ConstantSized for CMSG_AUTOBANK_ITEM {}

impl MaximumPossibleSized for CMSG_AUTOBANK_ITEM {
    fn maximum_possible_size() -> usize {
        0
        + 1 // bag_index: u8
        + 1 // slot_index: u8
    }
}

