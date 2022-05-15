use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{StableResult, StableResultError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_STABLE_RESULT {
    pub result: StableResult,
}

impl ServerMessageWrite for SMSG_STABLE_RESULT {}

impl MessageBody for SMSG_STABLE_RESULT {
    const OPCODE: u16 = 0x0273;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_STABLE_RESULTError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // result: StableResult
        let result: StableResult = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            result,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // result: StableResult
        w.write_all(&(self.result.as_int() as u8).to_le_bytes())?;

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
            // result: StableResult
            let result: StableResult = crate::util::tokio_read_u8_le(r).await?.try_into()?;

            Ok(Self {
                result,
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
            // result: StableResult
            w.write_all(&(self.result.as_int() as u8).to_le_bytes()).await?;

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
            // result: StableResult
            let result: StableResult = crate::util::astd_read_u8_le(r).await?.try_into()?;

            Ok(Self {
                result,
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
            // result: StableResult
            w.write_all(&(self.result.as_int() as u8).to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl ConstantSized for SMSG_STABLE_RESULT {}

impl MaximumPossibleSized for SMSG_STABLE_RESULT {
    fn maximum_possible_size() -> usize {
        0
        + 1 // result: StableResult
    }
}

#[derive(Debug)]
pub enum SMSG_STABLE_RESULTError {
    Io(std::io::Error),
    StableResult(StableResultError),
}

impl std::error::Error for SMSG_STABLE_RESULTError {}
impl std::fmt::Display for SMSG_STABLE_RESULTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::StableResult(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_STABLE_RESULTError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<StableResultError> for SMSG_STABLE_RESULTError {
    fn from(e: StableResultError) -> Self {
        Self::StableResult(e)
    }
}

