use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{SpellCastResult, SpellCastResultError};
use crate::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spell_failure.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spell_failure.wowm#L3):
/// ```text
/// smsg SMSG_SPELL_FAILURE = 0x133 {
///     u64 guid;
///     u32 spell_id;
///     SpellCastResult result;
/// }
/// ```
pub struct SMSG_SPELL_FAILURE {
    pub guid: u64,
    pub spell_id: u32,
    pub result: SpellCastResult,
}

impl WorldServerMessageWrite for SMSG_SPELL_FAILURE {
    const OPCODE: u16 = 0x133;

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
impl WorldMessageBody for SMSG_SPELL_FAILURE {
    type Error = SMSG_SPELL_FAILUREError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: u64
        let guid = crate::util::read_u64_le(r)?;

        // spell_id: u32
        let spell_id = crate::util::read_u32_le(r)?;

        // result: SpellCastResult
        let result = SpellCastResult::read(r)?;

        Ok(Self {
            guid,
            spell_id,
            result,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: u64
        w.write_all(&self.guid.to_le_bytes())?;

        // spell_id: u32
        w.write_all(&self.spell_id.to_le_bytes())?;

        // result: SpellCastResult
        self.result.write(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_SPELL_FAILURE {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_SPELL_FAILURE {
    fn maximum_possible_size() -> usize {
        8 // guid: u64
        + 4 // spell_id: u32
        + SpellCastResult::size() // result: SpellCastResult
    }
}

#[derive(Debug)]
pub enum SMSG_SPELL_FAILUREError {
    Io(std::io::Error),
    SpellCastResult(SpellCastResultError),
}

impl std::error::Error for SMSG_SPELL_FAILUREError {}
impl std::fmt::Display for SMSG_SPELL_FAILUREError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::SpellCastResult(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_SPELL_FAILUREError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<SpellCastResultError> for SMSG_SPELL_FAILUREError {
    fn from(e: SpellCastResultError) -> Self {
        Self::SpellCastResult(e)
    }
}

