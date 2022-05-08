use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{PetCommandState, PetCommandStateError};
use crate::world::v1::v12::{PetReactState, PetReactStateError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_PET_MODE {
    pub guid: Guid,
    pub react_state: PetReactState,
    pub command_state: PetCommandState,
    pub unknown1: u8,
    pub pet_enabled: u8,
}

impl ServerMessageWrite for SMSG_PET_MODE {}

impl MessageBody for SMSG_PET_MODE {
    const OPCODE: u16 = 0x017a;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_PET_MODEError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // react_state: PetReactState
        let react_state = PetReactState::read(r)?;

        // command_state: PetCommandState
        let command_state = PetCommandState::read(r)?;

        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(r)?;

        // pet_enabled: u8
        let pet_enabled = crate::util::read_u8_le(r)?;

        Ok(Self {
            guid,
            react_state,
            command_state,
            unknown1,
            pet_enabled,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // react_state: PetReactState
        self.react_state.write(w)?;

        // command_state: PetCommandState
        self.command_state.write(w)?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        // pet_enabled: u8
        w.write_all(&self.pet_enabled.to_le_bytes())?;

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
            // guid: Guid
            let guid = Guid::tokio_read(r).await?;

            // react_state: PetReactState
            let react_state = PetReactState::tokio_read(r).await?;

            // command_state: PetCommandState
            let command_state = PetCommandState::tokio_read(r).await?;

            // unknown1: u8
            let unknown1 = crate::util::tokio_read_u8_le(r).await?;

            // pet_enabled: u8
            let pet_enabled = crate::util::tokio_read_u8_le(r).await?;

            Ok(Self {
                guid,
                react_state,
                command_state,
                unknown1,
                pet_enabled,
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
            // guid: Guid
            self.guid.tokio_write(w).await?;

            // react_state: PetReactState
            self.react_state.tokio_write(w).await?;

            // command_state: PetCommandState
            self.command_state.tokio_write(w).await?;

            // unknown1: u8
            w.write_all(&self.unknown1.to_le_bytes()).await?;

            // pet_enabled: u8
            w.write_all(&self.pet_enabled.to_le_bytes()).await?;

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
            // guid: Guid
            let guid = Guid::astd_read(r).await?;

            // react_state: PetReactState
            let react_state = PetReactState::astd_read(r).await?;

            // command_state: PetCommandState
            let command_state = PetCommandState::astd_read(r).await?;

            // unknown1: u8
            let unknown1 = crate::util::astd_read_u8_le(r).await?;

            // pet_enabled: u8
            let pet_enabled = crate::util::astd_read_u8_le(r).await?;

            Ok(Self {
                guid,
                react_state,
                command_state,
                unknown1,
                pet_enabled,
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
            // guid: Guid
            self.guid.astd_write(w).await?;

            // react_state: PetReactState
            self.react_state.astd_write(w).await?;

            // command_state: PetCommandState
            self.command_state.astd_write(w).await?;

            // unknown1: u8
            w.write_all(&self.unknown1.to_le_bytes()).await?;

            // pet_enabled: u8
            w.write_all(&self.pet_enabled.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl ConstantSized for SMSG_PET_MODE {}

impl MaximumPossibleSized for SMSG_PET_MODE {
    fn maximum_possible_size() -> usize {
        0
        + 8 // guid: Guid
        + 1 // react_state: PetReactState
        + 1 // command_state: PetCommandState
        + 1 // unknown1: u8
        + 1 // pet_enabled: u8
    }
}

#[derive(Debug)]
pub enum SMSG_PET_MODEError {
    Io(std::io::Error),
    PetCommandState(PetCommandStateError),
    PetReactState(PetReactStateError),
}

impl std::error::Error for SMSG_PET_MODEError {}
impl std::fmt::Display for SMSG_PET_MODEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::PetCommandState(i) => i.fmt(f),
            Self::PetReactState(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_PET_MODEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<PetCommandStateError> for SMSG_PET_MODEError {
    fn from(e: PetCommandStateError) -> Self {
        Self::PetCommandState(e)
    }
}

impl From<PetReactStateError> for SMSG_PET_MODEError {
    fn from(e: PetReactStateError) -> Self {
        Self::PetReactState(e)
    }
}

