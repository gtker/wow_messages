use std::convert::{TryFrom, TryInto};
use crate::world::helper::Guid;
use crate::world::v1::v12::{AuraLog, AuraLogError};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/needs_else_if_else.wowm:462`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/needs_else_if_else.wowm#L462):
/// ```text
/// smsg SMSG_PERIODICAURALOG = 0x24E {
///     PackedGuid target;
///     PackedGuid caster;
///     u32 spell;
///     u32 amount_of_auras;
///     AuraLog[amount_of_auras] auras;
/// }
/// ```
pub struct SMSG_PERIODICAURALOG {
    pub target: Guid,
    pub caster: Guid,
    pub spell: u32,
    pub amount_of_auras: u32,
    pub auras: Vec<AuraLog>,
}

impl WorldServerMessageWrite for SMSG_PERIODICAURALOG {
    const OPCODE: u16 = 0x24e;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (self.size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (self.size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_PERIODICAURALOG {
    type Error = SMSG_PERIODICAURALOGError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // target: PackedGuid
        let target = Guid::read_packed(r)?;

        // caster: PackedGuid
        let caster = Guid::read_packed(r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        // amount_of_auras: u32
        let amount_of_auras = crate::util::read_u32_le(r)?;

        // auras: AuraLog[amount_of_auras]
        let mut auras = Vec::with_capacity(amount_of_auras as usize);
        for i in 0..amount_of_auras {
            auras.push(AuraLog::read(r)?);
        }

        Ok(Self {
            target,
            caster,
            spell,
            amount_of_auras,
            auras,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // target: PackedGuid
        self.target.write_packed(w)?;

        // caster: PackedGuid
        self.caster.write_packed(w)?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // amount_of_auras: u32
        w.write_all(&(self.auras.len() as u32).to_le_bytes())?;

        // auras: AuraLog[amount_of_auras]
        for i in self.auras.iter() {
            i.write(w)?;
        }

        Ok(())
    }
}

impl VariableSized for SMSG_PERIODICAURALOG {
    fn size(&self) -> usize {
        self.target.size() // target: PackedGuid
        + self.caster.size() // caster: PackedGuid
        + 4 // spell: u32
        + 4 // amount_of_auras: u32
        + self.auras.iter().fold(0, |acc, x| acc + x.size()) // auras: AuraLog[amount_of_auras]
    }
}

impl MaximumPossibleSized for SMSG_PERIODICAURALOG {
    fn maximum_possible_size() -> usize {
        9 // target: PackedGuid
        + 9 // caster: PackedGuid
        + 4 // spell: u32
        + 4 // amount_of_auras: u32
        + 4294967295 * AuraLog::maximum_possible_size() // auras: AuraLog[amount_of_auras]
    }
}

#[derive(Debug)]
pub enum SMSG_PERIODICAURALOGError {
    Io(std::io::Error),
    AuraLog(AuraLogError),
}

impl std::error::Error for SMSG_PERIODICAURALOGError {}
impl std::fmt::Display for SMSG_PERIODICAURALOGError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::AuraLog(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_PERIODICAURALOGError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<AuraLogError> for SMSG_PERIODICAURALOGError {
    fn from(e: AuraLogError) -> Self {
        Self::AuraLog(e)
    }
}

