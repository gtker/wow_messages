use std::convert::{TryFrom, TryInto};
use crate::world::helper::Guid;
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/needs_packed_guid.wowm:112`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/needs_packed_guid.wowm#L112):
/// ```text
/// smsg SMSG_SPELLHEALLOG = 0x150 {
///     PackedGuid victim_guid;
///     PackedGuid caster_guid;
///     u32 spell_id;
///     u32 damage;
///     u8 critical;
/// }
/// ```
pub struct SMSG_SPELLHEALLOG {
    pub victim_guid: Guid,
    pub caster_guid: Guid,
    pub spell_id: u32,
    pub damage: u32,
    pub critical: u8,
}

impl WorldServerMessageWrite for SMSG_SPELLHEALLOG {
    const OPCODE: u16 = 0x150;

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
impl WorldMessageBody for SMSG_SPELLHEALLOG {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // victim_guid: PackedGuid
        let victim_guid = Guid::read_packed(r)?;

        // caster_guid: PackedGuid
        let caster_guid = Guid::read_packed(r)?;

        // spell_id: u32
        let spell_id = crate::util::read_u32_le(r)?;

        // damage: u32
        let damage = crate::util::read_u32_le(r)?;

        // critical: u8
        let critical = crate::util::read_u8_le(r)?;

        Ok(Self {
            victim_guid,
            caster_guid,
            spell_id,
            damage,
            critical,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // victim_guid: PackedGuid
        self.victim_guid.write_packed(w)?;

        // caster_guid: PackedGuid
        self.caster_guid.write_packed(w)?;

        // spell_id: u32
        w.write_all(&self.spell_id.to_le_bytes())?;

        // damage: u32
        w.write_all(&self.damage.to_le_bytes())?;

        // critical: u8
        w.write_all(&self.critical.to_le_bytes())?;

        Ok(())
    }
}

impl VariableSized for SMSG_SPELLHEALLOG {
    fn size(&self) -> usize {
        self.victim_guid.size() // victim_guid: PackedGuid
        + self.caster_guid.size() // caster_guid: PackedGuid
        + 4 // spell_id: u32
        + 4 // damage: u32
        + 1 // critical: u8
    }
}

impl MaximumPossibleSized for SMSG_SPELLHEALLOG {
    fn maximum_possible_size() -> usize {
        9 // victim_guid: PackedGuid
        + 9 // caster_guid: PackedGuid
        + 4 // spell_id: u32
        + 4 // damage: u32
        + 1 // critical: u8
    }
}

