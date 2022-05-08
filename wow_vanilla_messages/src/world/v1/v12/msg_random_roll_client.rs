use std::convert::{TryFrom, TryInto};
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct MSG_RANDOM_ROLL_Client {
    pub minimum: u32,
    pub maximum: u32,
}

impl ClientMessageWrite for MSG_RANDOM_ROLL_Client {}

impl MessageBody for MSG_RANDOM_ROLL_Client {
    const OPCODE: u16 = 0x01fb;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // minimum: u32
        let minimum = crate::util::read_u32_le(r)?;

        // maximum: u32
        let maximum = crate::util::read_u32_le(r)?;

        Ok(Self {
            minimum,
            maximum,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // minimum: u32
        w.write_all(&self.minimum.to_le_bytes())?;

        // maximum: u32
        w.write_all(&self.maximum.to_le_bytes())?;

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
            // minimum: u32
            let minimum = crate::util::tokio_read_u32_le(r).await?;

            // maximum: u32
            let maximum = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                minimum,
                maximum,
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
            // minimum: u32
            w.write_all(&self.minimum.to_le_bytes()).await?;

            // maximum: u32
            w.write_all(&self.maximum.to_le_bytes()).await?;

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
            // minimum: u32
            let minimum = crate::util::astd_read_u32_le(r).await?;

            // maximum: u32
            let maximum = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                minimum,
                maximum,
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
            // minimum: u32
            w.write_all(&self.minimum.to_le_bytes()).await?;

            // maximum: u32
            w.write_all(&self.maximum.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl ConstantSized for MSG_RANDOM_ROLL_Client {}

impl MaximumPossibleSized for MSG_RANDOM_ROLL_Client {
    fn maximum_possible_size() -> usize {
        0
        + 4 // minimum: u32
        + 4 // maximum: u32
    }
}

