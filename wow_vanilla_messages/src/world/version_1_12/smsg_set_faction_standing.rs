use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::Faction;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_SET_FACTION_STANDING {
    pub factions: Vec<Faction>,
}

impl ServerMessage for SMSG_SET_FACTION_STANDING {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // amount_of_factions: u32
        w.write_all(&(self.factions.len() as u32).to_le_bytes())?;

        // factions: Faction[amount_of_factions]
        for i in self.factions.iter() {
            i.write_into_vec(w)?;
        }

        Ok(())
    }
    const OPCODE: u16 = 0x0124;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // amount_of_factions: u32
        let amount_of_factions = crate::util::read_u32_le(r)?;

        // factions: Faction[amount_of_factions]
        let mut factions = Vec::with_capacity(amount_of_factions as usize);
        for i in 0..amount_of_factions {
            factions.push(Faction::read(r)?);
        }

        Ok(Self {
            factions,
        })
    }

}

impl SMSG_SET_FACTION_STANDING {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_factions: u32
        + self.factions.len() * 8 // factions: Faction[amount_of_factions]
    }
}

