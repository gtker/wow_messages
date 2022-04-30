use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{Area, AreaError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use crate::AsyncReadWrite;
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_PLAYERBOUND {
    pub guid: Guid,
    pub area: Area,
}

impl ServerMessageWrite for SMSG_PLAYERBOUND {
    const OPCODE: u16 = 0x158;

    fn size_without_size_field(&self) -> u16 {
        Self::size() as u16
    }

}

impl MessageBody for SMSG_PLAYERBOUND {
    type Error = SMSG_PLAYERBOUNDError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // area: Area
        let area = Area::read(r)?;

        Ok(Self {
            guid,
            area,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // area: Area
        self.area.write(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_PLAYERBOUND {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_PLAYERBOUND {
    fn maximum_possible_size() -> usize {
        8 // guid: Guid
        + Area::size() // area: Area
    }
}

#[derive(Debug)]
pub enum SMSG_PLAYERBOUNDError {
    Io(std::io::Error),
    Area(AreaError),
}

impl std::error::Error for SMSG_PLAYERBOUNDError {}
impl std::fmt::Display for SMSG_PLAYERBOUNDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Area(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_PLAYERBOUNDError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<AreaError> for SMSG_PLAYERBOUNDError {
    fn from(e: AreaError) -> Self {
        Self::Area(e)
    }
}

