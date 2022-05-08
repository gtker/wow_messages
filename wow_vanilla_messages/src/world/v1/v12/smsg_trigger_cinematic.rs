use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{CinematicSequenceId, CinematicSequenceIdError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_TRIGGER_CINEMATIC {
    pub cinematic_sequence_id: CinematicSequenceId,
}

impl ServerMessageWrite for SMSG_TRIGGER_CINEMATIC {}

impl MessageBody for SMSG_TRIGGER_CINEMATIC {
    const OPCODE: u16 = 0x00fa;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_TRIGGER_CINEMATICError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // cinematic_sequence_id: CinematicSequenceId
        let cinematic_sequence_id = CinematicSequenceId::read(r)?;

        Ok(Self {
            cinematic_sequence_id,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // cinematic_sequence_id: CinematicSequenceId
        self.cinematic_sequence_id.write(w)?;

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
            // cinematic_sequence_id: CinematicSequenceId
            let cinematic_sequence_id = CinematicSequenceId::tokio_read(r).await?;

            Ok(Self {
                cinematic_sequence_id,
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
            // cinematic_sequence_id: CinematicSequenceId
            self.cinematic_sequence_id.tokio_write(w).await?;

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
            // cinematic_sequence_id: CinematicSequenceId
            let cinematic_sequence_id = CinematicSequenceId::astd_read(r).await?;

            Ok(Self {
                cinematic_sequence_id,
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
            // cinematic_sequence_id: CinematicSequenceId
            self.cinematic_sequence_id.astd_write(w).await?;

            Ok(())
        })
    }

}

impl ConstantSized for SMSG_TRIGGER_CINEMATIC {}

impl MaximumPossibleSized for SMSG_TRIGGER_CINEMATIC {
    fn maximum_possible_size() -> usize {
        0
        + 4 // cinematic_sequence_id: CinematicSequenceId
    }
}

#[derive(Debug)]
pub enum SMSG_TRIGGER_CINEMATICError {
    Io(std::io::Error),
    CinematicSequenceId(CinematicSequenceIdError),
}

impl std::error::Error for SMSG_TRIGGER_CINEMATICError {}
impl std::fmt::Display for SMSG_TRIGGER_CINEMATICError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::CinematicSequenceId(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_TRIGGER_CINEMATICError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<CinematicSequenceIdError> for SMSG_TRIGGER_CINEMATICError {
    fn from(e: CinematicSequenceIdError) -> Self {
        Self::CinematicSequenceId(e)
    }
}

