use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{EnvironmentalDamageType, EnvironmentalDamageTypeError};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_environmentaldamagelog.wowm:12`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_environmentaldamagelog.wowm#L12):
/// ```text
/// smsg SMSG_ENVIRONMENTALDAMAGELOG = 0x1FC {
///     u64 guid;
///     EnvironmentalDamageType damage_type;
///     u32 damage;
///     u32 absorb;
///     u32 resist;
/// }
/// ```
pub struct SMSG_ENVIRONMENTALDAMAGELOG {
    pub guid: u64,
    pub damage_type: EnvironmentalDamageType,
    pub damage: u32,
    pub absorb: u32,
    pub resist: u32,
}

impl WorldServerMessageWrite for SMSG_ENVIRONMENTALDAMAGELOG {
    const OPCODE: u16 = 0x1fc;

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
impl WorldMessageBody for SMSG_ENVIRONMENTALDAMAGELOG {
    type Error = SMSG_ENVIRONMENTALDAMAGELOGError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: u64
        let guid = crate::util::read_u64_le(r)?;

        // damage_type: EnvironmentalDamageType
        let damage_type = EnvironmentalDamageType::read(r)?;

        // damage: u32
        let damage = crate::util::read_u32_le(r)?;

        // absorb: u32
        let absorb = crate::util::read_u32_le(r)?;

        // resist: u32
        let resist = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            damage_type,
            damage,
            absorb,
            resist,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: u64
        w.write_all(&self.guid.to_le_bytes())?;

        // damage_type: EnvironmentalDamageType
        self.damage_type.write(w)?;

        // damage: u32
        w.write_all(&self.damage.to_le_bytes())?;

        // absorb: u32
        w.write_all(&self.absorb.to_le_bytes())?;

        // resist: u32
        w.write_all(&self.resist.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_ENVIRONMENTALDAMAGELOG {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_ENVIRONMENTALDAMAGELOG {
    fn maximum_possible_size() -> usize {
        8 // guid: u64
        + EnvironmentalDamageType::size() // damage_type: EnvironmentalDamageType
        + 4 // damage: u32
        + 4 // absorb: u32
        + 4 // resist: u32
    }
}

#[derive(Debug)]
pub enum SMSG_ENVIRONMENTALDAMAGELOGError {
    Io(std::io::Error),
    EnvironmentalDamageType(EnvironmentalDamageTypeError),
}

impl std::error::Error for SMSG_ENVIRONMENTALDAMAGELOGError {}
impl std::fmt::Display for SMSG_ENVIRONMENTALDAMAGELOGError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::EnvironmentalDamageType(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_ENVIRONMENTALDAMAGELOGError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<EnvironmentalDamageTypeError> for SMSG_ENVIRONMENTALDAMAGELOGError {
    fn from(e: EnvironmentalDamageTypeError) -> Self {
        Self::EnvironmentalDamageType(e)
    }
}

