use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{SheathState, SheathStateError};
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
pub struct CMSG_SETSHEATHED {
    pub sheathed: SheathState,
}

impl ClientMessageWrite for CMSG_SETSHEATHED {}

impl MessageBody for CMSG_SETSHEATHED {
    const OPCODE: u16 = 0x01e0;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = CMSG_SETSHEATHEDError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // sheathed: SheathState
        let sheathed = SheathState::read_u32_le(r)?;

        Ok(Self {
            sheathed,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // sheathed: SheathState
        self.sheathed.write_u32_le(w)?;

        Ok(())
    }
}

impl ConstantSized for CMSG_SETSHEATHED {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_SETSHEATHED {
    fn maximum_possible_size() -> usize {
        4 // sheathed: SheathState upcasted to u32
    }
}

#[derive(Debug)]
pub enum CMSG_SETSHEATHEDError {
    Io(std::io::Error),
    SheathState(SheathStateError),
}

impl std::error::Error for CMSG_SETSHEATHEDError {}
impl std::fmt::Display for CMSG_SETSHEATHEDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::SheathState(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_SETSHEATHEDError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<SheathStateError> for CMSG_SETSHEATHEDError {
    fn from(e: SheathStateError) -> Self {
        Self::SheathState(e)
    }
}

