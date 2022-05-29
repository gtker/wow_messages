use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_SPELLDISPELLOG {
    pub victim: Guid,
    pub caster: Guid,
    pub spells: Vec<u32>,
}

impl ServerMessage for SMSG_SPELLDISPELLOG {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // victim: Guid
        w.write_all(&self.victim.guid().to_le_bytes())?;

        // caster: Guid
        w.write_all(&self.caster.guid().to_le_bytes())?;

        // amount_of_spells: u32
        w.write_all(&(self.spells.len() as u32).to_le_bytes())?;

        // spells: u32[amount_of_spells]
        for i in self.spells.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
    const OPCODE: u16 = 0x027b;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // victim: Guid
        let victim = Guid::read(r)?;

        // caster: Guid
        let caster = Guid::read(r)?;

        // amount_of_spells: u32
        let amount_of_spells = crate::util::read_u32_le(r)?;

        // spells: u32[amount_of_spells]
        let mut spells = Vec::with_capacity(amount_of_spells as usize);
        for i in 0..amount_of_spells {
            spells.push(crate::util::read_u32_le(r)?);
        }

        Ok(Self {
            victim,
            caster,
            spells,
        })
    }

}

impl SMSG_SPELLDISPELLOG {
    pub(crate) fn size(&self) -> usize {
        8 // victim: Guid
        + 8 // caster: Guid
        + 4 // amount_of_spells: u32
        + self.spells.len() * core::mem::size_of::<u32>() // spells: u32[amount_of_spells]
    }
}

