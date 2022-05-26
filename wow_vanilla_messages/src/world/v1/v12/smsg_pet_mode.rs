use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::PetCommandState;
use crate::world::v1::v12::PetReactState;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_PET_MODE {
    pub guid: Guid,
    pub react_state: PetReactState,
    pub command_state: PetCommandState,
    pub unknown1: u8,
    pub pet_enabled: u8,
}

impl SMSG_PET_MODE {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 12], std::io::Error> {
        let mut array_w = [0u8; 12];
        let mut w = array_w.as_mut_slice();
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // react_state: PetReactState
        w.write_all(&(self.react_state.as_int() as u8).to_le_bytes())?;

        // command_state: PetCommandState
        w.write_all(&(self.command_state.as_int() as u8).to_le_bytes())?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        // pet_enabled: u8
        w.write_all(&self.pet_enabled.to_le_bytes())?;

        Ok(array_w)
    }
}

impl ServerMessage for SMSG_PET_MODE {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // react_state: PetReactState
        w.write_all(&(self.react_state.as_int() as u8).to_le_bytes())?;

        // command_state: PetCommandState
        w.write_all(&(self.command_state.as_int() as u8).to_le_bytes())?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        // pet_enabled: u8
        w.write_all(&self.pet_enabled.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x017a;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        12
    }

    type Error = SMSG_PET_MODEError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // react_state: PetReactState
        let react_state: PetReactState = crate::util::read_u8_le(r)?.try_into()?;

        // command_state: PetCommandState
        let command_state: PetCommandState = crate::util::read_u8_le(r)?.try_into()?;

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

            // react_state: PetReactState
            let react_state: PetReactState = crate::util::tokio_read_u8_le(r).await?.try_into()?;

            // command_state: PetCommandState
            let command_state: PetCommandState = crate::util::tokio_read_u8_le(r).await?.try_into()?;

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

            // react_state: PetReactState
            let react_state: PetReactState = crate::util::astd_read_u8_le(r).await?.try_into()?;

            // command_state: PetCommandState
            let command_state: PetCommandState = crate::util::astd_read_u8_le(r).await?.try_into()?;

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

}

#[derive(Debug)]
pub enum SMSG_PET_MODEError {
    Io(std::io::Error),
    Enum(crate::errors::EnumError),
}

impl std::error::Error for SMSG_PET_MODEError {}
impl std::fmt::Display for SMSG_PET_MODEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Enum(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_PET_MODEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<crate::errors::EnumError> for SMSG_PET_MODEError {
    fn from(e: crate::errors::EnumError) -> Self {
        Self::Enum(e)
    }
}

