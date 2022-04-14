use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::CooldownSpell;
use crate::world::v1::v12::InitialSpell;
use crate::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_INITIAL_SPELLS {
    pub unknown1: u8,
    pub initial_spells: Vec<InitialSpell>,
    pub cooldowns: Vec<CooldownSpell>,
}

impl WorldServerMessageWrite for SMSG_INITIAL_SPELLS {
    const OPCODE: u16 = 0x12a;

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
impl WorldMessageBody for SMSG_INITIAL_SPELLS {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(r)?;

        // spell_count: u16
        let spell_count = crate::util::read_u16_le(r)?;

        // initial_spells: InitialSpell[spell_count]
        let mut initial_spells = Vec::with_capacity(spell_count as usize);
        for i in 0..spell_count {
            initial_spells.push(InitialSpell::read(r)?);
        }

        // cooldown_count: u16
        let cooldown_count = crate::util::read_u16_le(r)?;

        // cooldowns: CooldownSpell[cooldown_count]
        let mut cooldowns = Vec::with_capacity(cooldown_count as usize);
        for i in 0..cooldown_count {
            cooldowns.push(CooldownSpell::read(r)?);
        }

        Ok(Self {
            unknown1,
            initial_spells,
            cooldowns,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        // spell_count: u16
        w.write_all(&(self.initial_spells.len() as u16).to_le_bytes())?;

        // initial_spells: InitialSpell[spell_count]
        for i in self.initial_spells.iter() {
            i.write(w)?;
        }

        // cooldown_count: u16
        w.write_all(&(self.cooldowns.len() as u16).to_le_bytes())?;

        // cooldowns: CooldownSpell[cooldown_count]
        for i in self.cooldowns.iter() {
            i.write(w)?;
        }

        Ok(())
    }
}

impl VariableSized for SMSG_INITIAL_SPELLS {
    fn size(&self) -> usize {
        1 // unknown1: u8
        + 2 // spell_count: u16
        + self.initial_spells.iter().fold(0, |acc, x| acc + InitialSpell::size()) // initial_spells: InitialSpell[spell_count]
        + 2 // cooldown_count: u16
        + self.cooldowns.iter().fold(0, |acc, x| acc + CooldownSpell::size()) // cooldowns: CooldownSpell[cooldown_count]
    }
}

impl MaximumPossibleSized for SMSG_INITIAL_SPELLS {
    fn maximum_possible_size() -> usize {
        1 // unknown1: u8
        + 2 // spell_count: u16
        + 65535 * InitialSpell::maximum_possible_size() // initial_spells: InitialSpell[spell_count]
        + 2 // cooldown_count: u16
        + 65535 * CooldownSpell::maximum_possible_size() // cooldowns: CooldownSpell[cooldown_count]
    }
}

