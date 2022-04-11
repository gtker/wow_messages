use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::Faction;
use crate::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_SET_FACTION_STANDING {
    pub amount_of_factions: u32,
    pub factions: Vec<Faction>,
}

impl WorldServerMessageWrite for SMSG_SET_FACTION_STANDING {
    const OPCODE: u16 = 0x124;

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
impl WorldMessageBody for SMSG_SET_FACTION_STANDING {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // amount_of_factions: u32
        let amount_of_factions = crate::util::read_u32_le(r)?;

        // factions: Faction[amount_of_factions]
        let mut factions = Vec::with_capacity(amount_of_factions as usize);
        for i in 0..amount_of_factions {
            factions.push(Faction::read(r)?);
        }

        Ok(Self {
            amount_of_factions,
            factions,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // amount_of_factions: u32
        w.write_all(&(self.factions.len() as u32).to_le_bytes())?;

        // factions: Faction[amount_of_factions]
        for i in self.factions.iter() {
            i.write(w)?;
        }

        Ok(())
    }
}

impl VariableSized for SMSG_SET_FACTION_STANDING {
    fn size(&self) -> usize {
        4 // amount_of_factions: u32
        + self.factions.iter().fold(0, |acc, x| acc + Faction::size()) // factions: Faction[amount_of_factions]
    }
}

impl MaximumPossibleSized for SMSG_SET_FACTION_STANDING {
    fn maximum_possible_size() -> usize {
        4 // amount_of_factions: u32
        + 4294967295 * Faction::maximum_possible_size() // factions: Faction[amount_of_factions]
    }
}

