use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::FactionInitializer;
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use crate::AsyncReadWrite;
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_INITIALIZE_FACTIONS {
    pub factions: Vec<FactionInitializer>,
}

impl ServerMessageWrite for SMSG_INITIALIZE_FACTIONS {
    const OPCODE: u16 = 0x122;

    fn size_without_size_field(&self) -> u16 {
        self.size() as u16
    }

}

impl MessageBody for SMSG_INITIALIZE_FACTIONS {
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

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // amount_of_factions: u32
        w.write_all(&(self.factions.len() as u32).to_le_bytes())?;

        // factions: FactionInitializer[amount_of_factions]
        for i in self.factions.iter() {
            i.write(w)?;
        }

        Ok(())
    }
}

impl VariableSized for SMSG_INITIALIZE_FACTIONS {
    fn size(&self) -> usize {
        4 // amount_of_factions: u32
        + self.factions.iter().fold(0, |acc, x| acc + FactionInitializer::size()) // factions: FactionInitializer[amount_of_factions]
    }
}

impl MaximumPossibleSized for SMSG_INITIALIZE_FACTIONS {
    fn maximum_possible_size() -> usize {
        4 // amount_of_factions: u32
        + 4294967295 * FactionInitializer::maximum_possible_size() // factions: FactionInitializer[amount_of_factions]
    }
}

