use std::convert::{TryFrom, TryInto};
use crate::helper::Guid;
use crate::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spell_update_chain_targets.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spell_update_chain_targets.wowm#L3):
/// ```text
/// smsg SMSG_SPELL_UPDATE_CHAIN_TARGETS = 0x330 {
///     Guid caster;
///     u32 spell;
///     u32 amount_of_targets;
///     Guid[amount_of_targets] targets;
/// }
/// ```
pub struct SMSG_SPELL_UPDATE_CHAIN_TARGETS {
    pub caster: Guid,
    pub spell: u32,
    pub amount_of_targets: u32,
    pub targets: Vec<Guid>,
}

impl WorldServerMessageWrite for SMSG_SPELL_UPDATE_CHAIN_TARGETS {
    const OPCODE: u16 = 0x330;

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
impl WorldMessageBody for SMSG_SPELL_UPDATE_CHAIN_TARGETS {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // caster: Guid
        let caster = Guid::read(r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        // amount_of_targets: u32
        let amount_of_targets = crate::util::read_u32_le(r)?;

        // targets: Guid[amount_of_targets]
        let mut targets = Vec::with_capacity(amount_of_targets as usize);
        for i in 0..amount_of_targets {
            targets.push(Guid::read(r)?);
        }

        Ok(Self {
            caster,
            spell,
            amount_of_targets,
            targets,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // caster: Guid
        self.caster.write(w)?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // amount_of_targets: u32
        w.write_all(&(self.targets.len() as u32).to_le_bytes())?;

        // targets: Guid[amount_of_targets]
        for i in self.targets.iter() {
            i.write(w)?;
        }

        Ok(())
    }
}

impl VariableSized for SMSG_SPELL_UPDATE_CHAIN_TARGETS {
    fn size(&self) -> usize {
        8 // caster: Guid
        + 4 // spell: u32
        + 4 // amount_of_targets: u32
        + 8 // targets: Guid[amount_of_targets]
    }
}

impl MaximumPossibleSized for SMSG_SPELL_UPDATE_CHAIN_TARGETS {
    fn maximum_possible_size() -> usize {
        8 // caster: Guid
        + 4 // spell: u32
        + 4 // amount_of_targets: u32
        + 4294967295 * 8 // targets: Guid[amount_of_targets]
    }
}

