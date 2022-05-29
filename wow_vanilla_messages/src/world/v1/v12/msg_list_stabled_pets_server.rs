use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::StabledPet;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct MSG_LIST_STABLED_PETS_Server {
    pub npc: Guid,
    pub stable_slots: u8,
    pub pets: Vec<StabledPet>,
}

impl ServerMessage for MSG_LIST_STABLED_PETS_Server {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // npc: Guid
        w.write_all(&self.npc.guid().to_le_bytes())?;

        // amount_of_pets: u8
        w.write_all(&(self.pets.len() as u8).to_le_bytes())?;

        // stable_slots: u8
        w.write_all(&self.stable_slots.to_le_bytes())?;

        // pets: StabledPet[amount_of_pets]
        for i in self.pets.iter() {
            i.write_into_vec(w)?;
        }

        Ok(())
    }
    const OPCODE: u16 = 0x026f;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = crate::errors::ParseError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // npc: Guid
        let npc = Guid::read(r)?;

        // amount_of_pets: u8
        let amount_of_pets = crate::util::read_u8_le(r)?;

        // stable_slots: u8
        let stable_slots = crate::util::read_u8_le(r)?;

        // pets: StabledPet[amount_of_pets]
        let mut pets = Vec::with_capacity(amount_of_pets as usize);
        for i in 0..amount_of_pets {
            pets.push(StabledPet::read(r)?);
        }

        Ok(Self {
            npc,
            stable_slots,
            pets,
        })
    }

}

impl MSG_LIST_STABLED_PETS_Server {
    pub(crate) fn size(&self) -> usize {
        8 // npc: Guid
        + 1 // amount_of_pets: u8
        + 1 // stable_slots: u8
        + self.pets.iter().fold(0, |acc, x| acc + x.size()) // pets: StabledPet[amount_of_pets]
    }
}

