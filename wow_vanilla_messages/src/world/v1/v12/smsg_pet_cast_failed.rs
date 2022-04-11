use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{SpellCastResult, SpellCastResultError};
use crate::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_PET_CAST_FAILED {
    pub id: u32,
    pub unknown1: u8,
    pub result: SpellCastResult,
}

impl WorldServerMessageWrite for SMSG_PET_CAST_FAILED {
    const OPCODE: u16 = 0x138;

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
impl WorldMessageBody for SMSG_PET_CAST_FAILED {
    type Error = SMSG_PET_CAST_FAILEDError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // id: u32
        let id = crate::util::read_u32_le(r)?;

        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(r)?;

        // result: SpellCastResult
        let result = SpellCastResult::read(r)?;

        Ok(Self {
            id,
            unknown1,
            result,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        // result: SpellCastResult
        self.result.write(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_PET_CAST_FAILED {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_PET_CAST_FAILED {
    fn maximum_possible_size() -> usize {
        4 // id: u32
        + 1 // unknown1: u8
        + SpellCastResult::size() // result: SpellCastResult
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

