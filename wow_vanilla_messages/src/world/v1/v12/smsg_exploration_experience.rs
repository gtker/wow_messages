use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Area, AreaError};
use crate::{WorldServerMessageWrite, MessageBody};
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
pub struct SMSG_EXPLORATION_EXPERIENCE {
    pub area: Area,
    pub experience: u32,
}

impl WorldServerMessageWrite for SMSG_EXPLORATION_EXPERIENCE {
    const OPCODE: u16 = 0x1f8;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (Self::size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (Self::size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl MessageBody for SMSG_EXPLORATION_EXPERIENCE {
    type Error = SMSG_EXPLORATION_EXPERIENCEError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // area: Area
        let area = Area::read(r)?;

        // experience: u32
        let experience = crate::util::read_u32_le(r)?;

        Ok(Self {
            area,
            experience,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // area: Area
        self.area.write(w)?;

        // experience: u32
        w.write_all(&self.experience.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_EXPLORATION_EXPERIENCE {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_EXPLORATION_EXPERIENCE {
    fn maximum_possible_size() -> usize {
        Area::size() // area: Area
        + 4 // experience: u32
    }
}

#[derive(Debug)]
pub enum SMSG_EXPLORATION_EXPERIENCEError {
    Io(std::io::Error),
    Area(AreaError),
}

impl std::error::Error for SMSG_EXPLORATION_EXPERIENCEError {}
impl std::fmt::Display for SMSG_EXPLORATION_EXPERIENCEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Area(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_EXPLORATION_EXPERIENCEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<AreaError> for SMSG_EXPLORATION_EXPERIENCEError {
    fn from(e: AreaError) -> Self {
        Self::Area(e)
    }
}

