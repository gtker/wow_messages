use std::convert::{TryFrom, TryInto};
use crate::world::helper::Guid;
use crate::world::v1::v12::{PowerType, PowerTypeError};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/remaining.wowm:2741`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/remaining.wowm#L2741):
/// ```text
/// smsg SMSG_SPELLENERGIZELOG = 0x151 {
///     PackedGuid victim_guid;
///     PackedGuid caster_guid;
///     u32 spell;
///     PowerType power;
///     u32 damage;
/// }
/// ```
pub struct SMSG_SPELLENERGIZELOG {
    pub victim_guid: Guid,
    pub caster_guid: Guid,
    pub spell: u32,
    pub power: PowerType,
    pub damage: u32,
}

impl WorldServerMessageWrite for SMSG_SPELLENERGIZELOG {
    const OPCODE: u16 = 0x151;

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
impl WorldMessageBody for SMSG_SPELLENERGIZELOG {
    type Error = SMSG_SPELLENERGIZELOGError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // victim_guid: PackedGuid
        let victim_guid = Guid::read_packed(r)?;

        // caster_guid: PackedGuid
        let caster_guid = Guid::read_packed(r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        // power: PowerType
        let power = PowerType::read(r)?;

        // damage: u32
        let damage = crate::util::read_u32_le(r)?;

        Ok(Self {
            victim_guid,
            caster_guid,
            spell,
            power,
            damage,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // victim_guid: PackedGuid
        self.victim_guid.write_packed(w)?;

        // caster_guid: PackedGuid
        self.caster_guid.write_packed(w)?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // power: PowerType
        self.power.write(w)?;

        // damage: u32
        w.write_all(&self.damage.to_le_bytes())?;

        Ok(())
    }
}

impl VariableSized for SMSG_SPELLENERGIZELOG {
    fn size(&self) -> usize {
        self.victim_guid.size() // victim_guid: PackedGuid
        + self.caster_guid.size() // caster_guid: PackedGuid
        + 4 // spell: u32
        + PowerType::size() // power: PowerType
        + 4 // damage: u32
    }
}

impl MaximumPossibleSized for SMSG_SPELLENERGIZELOG {
    fn maximum_possible_size() -> usize {
        9 // victim_guid: PackedGuid
        + 9 // caster_guid: PackedGuid
        + 4 // spell: u32
        + PowerType::maximum_possible_size() // power: PowerType
        + 4 // damage: u32
    }
}

#[derive(Debug)]
pub enum SMSG_SPELLENERGIZELOGError {
    Io(std::io::Error),
    PowerType(PowerTypeError),
}

impl std::error::Error for SMSG_SPELLENERGIZELOGError {}
impl std::fmt::Display for SMSG_SPELLENERGIZELOGError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::PowerType(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_SPELLENERGIZELOGError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<PowerTypeError> for SMSG_SPELLENERGIZELOGError {
    fn from(e: PowerTypeError) -> Self {
        Self::PowerType(e)
    }
}

