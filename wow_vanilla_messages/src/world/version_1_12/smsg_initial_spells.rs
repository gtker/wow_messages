use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::CooldownSpell;
use crate::world::version_1_12::InitialSpell;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_INITIAL_SPELLS {
    pub unknown1: u8,
    pub initial_spells: Vec<InitialSpell>,
    pub cooldowns: Vec<CooldownSpell>,
}

impl ServerMessage for SMSG_INITIAL_SPELLS {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        // spell_count: u16
        w.write_all(&(self.initial_spells.len() as u16).to_le_bytes())?;

        // initial_spells: InitialSpell[spell_count]
        for i in self.initial_spells.iter() {
            i.write_into_vec(w)?;
        }

        // cooldown_count: u16
        w.write_all(&(self.cooldowns.len() as u16).to_le_bytes())?;

        // cooldowns: CooldownSpell[cooldown_count]
        for i in self.cooldowns.iter() {
            i.write_into_vec(w)?;
        }

        Ok(())
    }
    const OPCODE: u16 = 0x012a;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
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

}

impl SMSG_INITIAL_SPELLS {
    pub(crate) fn size(&self) -> usize {
        1 // unknown1: u8
        + 2 // spell_count: u16
        + self.initial_spells.len() * 4 // initial_spells: InitialSpell[spell_count]
        + 2 // cooldown_count: u16
        + self.cooldowns.len() * 14 // cooldowns: CooldownSpell[cooldown_count]
    }
}

