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

impl MSG_LIST_STABLED_PETS_Server {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // npc: Guid
        w.write_all(&self.npc.guid().to_le_bytes())?;

        // amount_of_pets: u8
        w.write_all(&(self.pets.len() as u8).to_le_bytes())?;

        // stable_slots: u8
        w.write_all(&self.stable_slots.to_le_bytes())?;

        // pets: StabledPet[amount_of_pets]
        for i in self.pets.iter() {
            w.write_all(&(i.as_bytes()?))?;
        }

        Ok(w)
    }
}

impl ServerMessage for MSG_LIST_STABLED_PETS_Server {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // npc: Guid
        w.write_all(&self.npc.guid().to_le_bytes())?;

        // amount_of_pets: u8
        w.write_all(&(self.pets.len() as u8).to_le_bytes())?;

        // stable_slots: u8
        w.write_all(&self.stable_slots.to_le_bytes())?;

        // pets: StabledPet[amount_of_pets]
        for i in self.pets.iter() {
            w.write_all(&(i.as_bytes()?))?;
        }

        Ok(())
    }
    const OPCODE: u16 = 0x026f;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = crate::errors::ParseError;

    #[cfg(feature = "sync")]
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

    #[cfg(feature = "tokio")]
    fn tokio_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // npc: Guid
            let npc = Guid::tokio_read(r).await?;

            // amount_of_pets: u8
            let amount_of_pets = crate::util::tokio_read_u8_le(r).await?;

            // stable_slots: u8
            let stable_slots = crate::util::tokio_read_u8_le(r).await?;

            // pets: StabledPet[amount_of_pets]
            let mut pets = Vec::with_capacity(amount_of_pets as usize);
            for i in 0..amount_of_pets {
                pets.push(StabledPet::tokio_read(r).await?);
            }

            Ok(Self {
                npc,
                stable_slots,
                pets,
            })
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // npc: Guid
            let npc = Guid::astd_read(r).await?;

            // amount_of_pets: u8
            let amount_of_pets = crate::util::astd_read_u8_le(r).await?;

            // stable_slots: u8
            let stable_slots = crate::util::astd_read_u8_le(r).await?;

            // pets: StabledPet[amount_of_pets]
            let mut pets = Vec::with_capacity(amount_of_pets as usize);
            for i in 0..amount_of_pets {
                pets.push(StabledPet::astd_read(r).await?);
            }

            Ok(Self {
                npc,
                stable_slots,
                pets,
            })
        })
    }

}

impl MSG_LIST_STABLED_PETS_Server {
    pub fn size(&self) -> usize {
        0
        + 8 // npc: Guid
        + 1 // amount_of_pets: u8
        + 1 // stable_slots: u8
        + self.pets.iter().fold(0, |acc, x| acc + x.size()) // pets: StabledPet[amount_of_pets]
    }
}

