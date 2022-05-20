use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{SpellCastResult, SpellCastResultError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_SPELL_FAILURE {
    pub guid: Guid,
    pub id: u32,
    pub result: SpellCastResult,
}

impl ServerMessageWrite for SMSG_SPELL_FAILURE {}

impl MessageBody for SMSG_SPELL_FAILURE {
    const OPCODE: u16 = 0x0133;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_SPELL_FAILUREError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // id: u32
        let id = crate::util::read_u32_le(r)?;

        // result: SpellCastResult
        let result: SpellCastResult = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            guid,
            id,
            result,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // result: SpellCastResult
        w.write_all(&(self.result.as_int() as u8).to_le_bytes())?;

        Ok(())
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
            // guid: Guid
            let guid = Guid::tokio_read(r).await?;

            // id: u32
            let id = crate::util::tokio_read_u32_le(r).await?;

            // result: SpellCastResult
            let result: SpellCastResult = crate::util::tokio_read_u8_le(r).await?.try_into()?;

            Ok(Self {
                guid,
                id,
                result,
            })
        })
    }

    #[cfg(feature = "tokio")]
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
            // guid: Guid
            w.write_all(&self.guid.guid().to_le_bytes()).await?;

            // id: u32
            w.write_all(&self.id.to_le_bytes()).await?;

            // result: SpellCastResult
            w.write_all(&(self.result.as_int() as u8).to_le_bytes()).await?;

            Ok(())
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
            // guid: Guid
            let guid = Guid::astd_read(r).await?;

            // id: u32
            let id = crate::util::astd_read_u32_le(r).await?;

            // result: SpellCastResult
            let result: SpellCastResult = crate::util::astd_read_u8_le(r).await?.try_into()?;

            Ok(Self {
                guid,
                id,
                result,
            })
        })
    }

    #[cfg(feature = "async-std")]
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
            // guid: Guid
            w.write_all(&self.guid.guid().to_le_bytes()).await?;

            // id: u32
            w.write_all(&self.id.to_le_bytes()).await?;

            // result: SpellCastResult
            w.write_all(&(self.result.as_int() as u8).to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl SMSG_SPELL_FAILURE {
    pub(crate) fn size() -> usize {
        0
        + 8 // guid: Guid
        + 4 // id: u32
        + 1 // result: SpellCastResult
    }
}

#[derive(Debug)]
pub enum SMSG_SPELL_FAILUREError {
    Io(std::io::Error),
    SpellCastResult(SpellCastResultError),
}

impl std::error::Error for SMSG_SPELL_FAILUREError {}
impl std::fmt::Display for SMSG_SPELL_FAILUREError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::SpellCastResult(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_SPELL_FAILUREError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<SpellCastResultError> for SMSG_SPELL_FAILUREError {
    fn from(e: SpellCastResultError) -> Self {
        Self::SpellCastResult(e)
    }
}

