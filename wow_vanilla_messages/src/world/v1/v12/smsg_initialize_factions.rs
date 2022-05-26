use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::FactionInitializer;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_INITIALIZE_FACTIONS {
    pub factions: Vec<FactionInitializer>,
}

impl SMSG_INITIALIZE_FACTIONS {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // amount_of_factions: u32
        w.write_all(&(self.factions.len() as u32).to_le_bytes())?;

        // factions: FactionInitializer[amount_of_factions]
        for i in self.factions.iter() {
            w.write_all(&(i.as_bytes()?))?;
        }

        Ok(w)
    }
}

impl ServerMessage for SMSG_INITIALIZE_FACTIONS {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // amount_of_factions: u32
        w.write_all(&(self.factions.len() as u32).to_le_bytes())?;

        // factions: FactionInitializer[amount_of_factions]
        for i in self.factions.iter() {
            w.write_all(&(i.as_bytes()?))?;
        }

        Ok(())
    }
    const OPCODE: u16 = 0x0122;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // amount_of_factions: u32
        let amount_of_factions = crate::util::read_u32_le(r)?;

        // factions: FactionInitializer[amount_of_factions]
        let mut factions = Vec::with_capacity(amount_of_factions as usize);
        for i in 0..amount_of_factions {
            factions.push(FactionInitializer::read(r)?);
        }

        Ok(Self {
            factions,
        })
    }

}

impl SMSG_INITIALIZE_FACTIONS {
    pub fn size(&self) -> usize {
        0
        + 4 // amount_of_factions: u32
        + self.factions.len() * 5 // factions: FactionInitializer[amount_of_factions]
    }
}

