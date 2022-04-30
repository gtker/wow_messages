use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{MountResult, MountResultError};
use crate::{ServerMessageWrite, MessageBody};
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
pub struct SMSG_DISMOUNTRESULT {
    pub result: MountResult,
}

impl ServerMessageWrite for SMSG_DISMOUNTRESULT {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_DISMOUNTRESULT {
    const OPCODE: u16 = 0x016f;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_DISMOUNTRESULTError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // result: MountResult
        let result = MountResult::read(r)?;

        Ok(Self {
            result,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // result: MountResult
        self.result.write(w)?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // result: MountResult
        let result = MountResult::tokio_read(r).await?;

        Ok(Self {
            result,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // result: MountResult
        self.result.tokio_write(w).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // result: MountResult
        let result = MountResult::astd_read(r).await?;

        Ok(Self {
            result,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // result: MountResult
        self.result.astd_write(w).await?;

        Ok(())
    }
}

impl ConstantSized for SMSG_DISMOUNTRESULT {}

impl MaximumPossibleSized for SMSG_DISMOUNTRESULT {
    fn maximum_possible_size() -> usize {
        MountResult::size() // result: MountResult
    }
}

#[derive(Debug)]
pub enum SMSG_DISMOUNTRESULTError {
    Io(std::io::Error),
    MountResult(MountResultError),
}

impl std::error::Error for SMSG_DISMOUNTRESULTError {}
impl std::fmt::Display for SMSG_DISMOUNTRESULTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::MountResult(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_DISMOUNTRESULTError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<MountResultError> for SMSG_DISMOUNTRESULTError {
    fn from(e: MountResultError) -> Self {
        Self::MountResult(e)
    }
}

