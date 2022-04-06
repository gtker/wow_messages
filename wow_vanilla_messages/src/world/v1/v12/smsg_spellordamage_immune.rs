use std::convert::{TryFrom, TryInto};
use crate::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spellordamage_immune.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spellordamage_immune.wowm#L3):
/// ```text
/// smsg SMSG_SPELLORDAMAGE_IMMUNE = 0x263 {
///     u64 caster_guid;
///     u64 target_guid;
///     u32 spell_id;
///     u8 unknown1;
/// }
/// ```
pub struct SMSG_SPELLORDAMAGE_IMMUNE {
    pub caster_guid: u64,
    pub target_guid: u64,
    pub spell_id: u32,
    pub unknown1: u8,
}

impl WorldServerMessageWrite for SMSG_SPELLORDAMAGE_IMMUNE {
    const OPCODE: u16 = 0x263;

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
impl WorldMessageBody for SMSG_SPELLORDAMAGE_IMMUNE {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // caster_guid: u64
        let caster_guid = crate::util::read_u64_le(r)?;

        // target_guid: u64
        let target_guid = crate::util::read_u64_le(r)?;

        // spell_id: u32
        let spell_id = crate::util::read_u32_le(r)?;

        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(r)?;

        Ok(Self {
            caster_guid,
            target_guid,
            spell_id,
            unknown1,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // caster_guid: u64
        w.write_all(&self.caster_guid.to_le_bytes())?;

        // target_guid: u64
        w.write_all(&self.target_guid.to_le_bytes())?;

        // spell_id: u32
        w.write_all(&self.spell_id.to_le_bytes())?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_SPELLORDAMAGE_IMMUNE {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_SPELLORDAMAGE_IMMUNE {
    fn maximum_possible_size() -> usize {
        8 // caster_guid: u64
        + 8 // target_guid: u64
        + 4 // spell_id: u32
        + 1 // unknown1: u8
    }
}

