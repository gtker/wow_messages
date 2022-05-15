use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{SpellCastResult, SpellCastResultError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_PET_CAST_FAILED {
    pub id: u32,
    pub unknown1: u8,
    pub result: SpellCastResult,
}

impl ServerMessageWrite for SMSG_PET_CAST_FAILED {}

impl MessageBody for SMSG_PET_CAST_FAILED {
    const OPCODE: u16 = 0x0138;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_PET_CAST_FAILEDError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // id: u32
        let id = crate::util::read_u32_le(r)?;

        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(r)?;

        // result: SpellCastResult
        let result: SpellCastResult = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            id,
            unknown1,
            result,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        // result: SpellCastResult
        crate::util::write_u8_le(w, self.result.as_int() as u8)?;

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
            // id: u32
            let id = crate::util::tokio_read_u32_le(r).await?;

            // unknown1: u8
            let unknown1 = crate::util::tokio_read_u8_le(r).await?;

            // result: SpellCastResult
            let result: SpellCastResult = crate::util::tokio_read_u8_le(r).await?.try_into()?;

            Ok(Self {
                id,
                unknown1,
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
            // id: u32
            w.write_all(&self.id.to_le_bytes()).await?;

            // unknown1: u8
            w.write_all(&self.unknown1.to_le_bytes()).await?;

            // result: SpellCastResult
            crate::util::tokio_write_u8_le(w, self.result.as_int() as u8).await?;

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
            // id: u32
            let id = crate::util::astd_read_u32_le(r).await?;

            // unknown1: u8
            let unknown1 = crate::util::astd_read_u8_le(r).await?;

            // result: SpellCastResult
            let result: SpellCastResult = crate::util::astd_read_u8_le(r).await?.try_into()?;

            Ok(Self {
                id,
                unknown1,
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
            // id: u32
            w.write_all(&self.id.to_le_bytes()).await?;

            // unknown1: u8
            w.write_all(&self.unknown1.to_le_bytes()).await?;

            // result: SpellCastResult
            crate::util::astd_write_u8_le(w, self.result.as_int() as u8).await?;

            Ok(())
        })
    }

}

impl ConstantSized for SMSG_PET_CAST_FAILED {}

impl MaximumPossibleSized for SMSG_PET_CAST_FAILED {
    fn maximum_possible_size() -> usize {
        0
        + 4 // id: u32
        + 1 // unknown1: u8
        + 1 // result: SpellCastResult
    }
}

#[derive(Debug)]
pub enum SMSG_PET_CAST_FAILEDError {
    Io(std::io::Error),
    SpellCastResult(SpellCastResultError),
}

impl std::error::Error for SMSG_PET_CAST_FAILEDError {}
impl std::fmt::Display for SMSG_PET_CAST_FAILEDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::SpellCastResult(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_PET_CAST_FAILEDError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<SpellCastResultError> for SMSG_PET_CAST_FAILEDError {
    fn from(e: SpellCastResultError) -> Self {
        Self::SpellCastResult(e)
    }
}

