use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{UnitStandState, UnitStandStateError};
use crate::{ClientMessageWrite, MessageBody};
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
pub struct CMSG_STANDSTATECHANGE {
    pub animation_state: UnitStandState,
}

impl ClientMessageWrite for CMSG_STANDSTATECHANGE {}

impl MessageBody for CMSG_STANDSTATECHANGE {
    const OPCODE: u16 = 0x0101;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = CMSG_STANDSTATECHANGEError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // animation_state: UnitStandState
        let animation_state = UnitStandState::read_u32_le(r)?;

        Ok(Self {
            animation_state,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // animation_state: UnitStandState
        self.animation_state.write_u32_le(w)?;

        Ok(())
    }
}

impl ConstantSized for CMSG_STANDSTATECHANGE {}

impl MaximumPossibleSized for CMSG_STANDSTATECHANGE {
    fn maximum_possible_size() -> usize {
        4 // animation_state: UnitStandState upcasted to u32
    }
}

#[derive(Debug)]
pub enum CMSG_STANDSTATECHANGEError {
    Io(std::io::Error),
    UnitStandState(UnitStandStateError),
}

impl std::error::Error for CMSG_STANDSTATECHANGEError {}
impl std::fmt::Display for CMSG_STANDSTATECHANGEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::UnitStandState(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_STANDSTATECHANGEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<UnitStandStateError> for CMSG_STANDSTATECHANGEError {
    fn from(e: UnitStandStateError) -> Self {
        Self::UnitStandState(e)
    }
}

