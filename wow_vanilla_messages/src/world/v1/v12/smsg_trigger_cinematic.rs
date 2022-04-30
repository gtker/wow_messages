use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{CinematicSequenceId, CinematicSequenceIdError};
use crate::{WorldServerMessageWrite, WorldMessageBody};
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
pub struct SMSG_TRIGGER_CINEMATIC {
    pub cinematic_sequence_id: CinematicSequenceId,
}

impl WorldServerMessageWrite for SMSG_TRIGGER_CINEMATIC {
    const OPCODE: u16 = 0xfa;

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
impl WorldMessageBody for SMSG_TRIGGER_CINEMATIC {
    type Error = SMSG_TRIGGER_CINEMATICError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // cinematic_sequence_id: CinematicSequenceId
        let cinematic_sequence_id = CinematicSequenceId::read(r)?;

        Ok(Self {
            cinematic_sequence_id,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // cinematic_sequence_id: CinematicSequenceId
        self.cinematic_sequence_id.write(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_TRIGGER_CINEMATIC {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_TRIGGER_CINEMATIC {
    fn maximum_possible_size() -> usize {
        CinematicSequenceId::size() // cinematic_sequence_id: CinematicSequenceId
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

