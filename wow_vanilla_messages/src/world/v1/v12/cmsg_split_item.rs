use std::convert::{TryFrom, TryInto};
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_SPLIT_ITEM {
    pub source_bag: u8,
    pub source_slot: u8,
    pub destination_bag: u8,
    pub destination_slot: u8,
    pub amount: u8,
}

impl ClientMessageWrite for CMSG_SPLIT_ITEM {}

impl MessageBody for CMSG_SPLIT_ITEM {
    const OPCODE: u16 = 0x010e;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
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

        // destination_slot: u8
        let destination_slot = crate::util::read_u8_le(r)?;

        // amount: u8
        let amount = crate::util::read_u8_le(r)?;

        Ok(Self {
            source_bag,
            source_slot,
            destination_bag,
            destination_slot,
            amount,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // source_bag: u8
        w.write_all(&self.source_bag.to_le_bytes())?;

        // source_slot: u8
        w.write_all(&self.source_slot.to_le_bytes())?;

        // destination_bag: u8
        w.write_all(&self.destination_bag.to_le_bytes())?;

        // destination_slot: u8
        w.write_all(&self.destination_slot.to_le_bytes())?;

        // amount: u8
        w.write_all(&self.amount.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
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

            // destination_slot: u8
            let destination_slot = crate::util::tokio_read_u8_le(r).await?;

            // amount: u8
            let amount = crate::util::tokio_read_u8_le(r).await?;

            Ok(Self {
                source_bag,
                source_slot,
                destination_bag,
                destination_slot,
                amount,
            })
        })
    }

    #[cfg(feature = "async_tokio")]
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
            // source_bag: u8
            w.write_all(&self.source_bag.to_le_bytes()).await?;

            // source_slot: u8
            w.write_all(&self.source_slot.to_le_bytes()).await?;

            // destination_bag: u8
            w.write_all(&self.destination_bag.to_le_bytes()).await?;

            // destination_slot: u8
            w.write_all(&self.destination_slot.to_le_bytes()).await?;

            // amount: u8
            w.write_all(&self.amount.to_le_bytes()).await?;

            Ok(())
        })
    }

    #[cfg(feature = "async_std")]
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

            // destination_slot: u8
            let destination_slot = crate::util::astd_read_u8_le(r).await?;

            // amount: u8
            let amount = crate::util::astd_read_u8_le(r).await?;

            Ok(Self {
                source_bag,
                source_slot,
                destination_bag,
                destination_slot,
                amount,
            })
        })
    }

    #[cfg(feature = "async_std")]
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
            // source_bag: u8
            w.write_all(&self.source_bag.to_le_bytes()).await?;

            // source_slot: u8
            w.write_all(&self.source_slot.to_le_bytes()).await?;

            // destination_bag: u8
            w.write_all(&self.destination_bag.to_le_bytes()).await?;

            // destination_slot: u8
            w.write_all(&self.destination_slot.to_le_bytes()).await?;

            // amount: u8
            w.write_all(&self.amount.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl ConstantSized for CMSG_SPLIT_ITEM {}

impl MaximumPossibleSized for CMSG_SPLIT_ITEM {
    fn maximum_possible_size() -> usize {
        0
        + 1 // source_bag: u8
        + 1 // source_slot: u8
        + 1 // destination_bag: u8
        + 1 // destination_slot: u8
        + 1 // amount: u8
    }
}

