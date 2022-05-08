use std::convert::{TryFrom, TryInto};
use crate::world::v1::v2::{WorldResult, WorldResultError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_CHAR_DELETE {
    pub result: WorldResult,
}

impl ServerMessageWrite for SMSG_CHAR_DELETE {}

impl MessageBody for SMSG_CHAR_DELETE {
    const OPCODE: u16 = 0x003c;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_CHAR_DELETEError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // result: WorldResult
        let result = WorldResult::read(r)?;

        Ok(Self {
            result,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // result: WorldResult
        self.result.write(w)?;

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
            // result: WorldResult
            let result = WorldResult::tokio_read(r).await?;

            Ok(Self {
                result,
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
            // result: WorldResult
            self.result.tokio_write(w).await?;

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
            // result: WorldResult
            let result = WorldResult::astd_read(r).await?;

            Ok(Self {
                result,
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
            // result: WorldResult
            self.result.astd_write(w).await?;

            Ok(())
        })
    }

}

impl ConstantSized for SMSG_CHAR_DELETE {}

impl MaximumPossibleSized for SMSG_CHAR_DELETE {
    fn maximum_possible_size() -> usize {
        0
        + 4 // result: WorldResult
    }
}

#[derive(Debug)]
pub enum SMSG_CHAR_DELETEError {
    Io(std::io::Error),
    WorldResult(WorldResultError),
}

impl std::error::Error for SMSG_CHAR_DELETEError {}
impl std::fmt::Display for SMSG_CHAR_DELETEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::WorldResult(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_CHAR_DELETEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<WorldResultError> for SMSG_CHAR_DELETEError {
    fn from(e: WorldResultError) -> Self {
        Self::WorldResult(e)
    }
}

