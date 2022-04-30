use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::FactionInitializer;
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
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

impl ServerMessageWrite for SMSG_INITIALIZE_FACTIONS {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_INITIALIZE_FACTIONS {
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

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // amount_of_factions: u32
        w.write_all(&(self.factions.len() as u32).to_le_bytes())?;

        // factions: FactionInitializer[amount_of_factions]
        for i in self.factions.iter() {
            i.write(w)?;
        }

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // amount_of_factions: u32
        let amount_of_factions = crate::util::tokio_read_u32_le(r).await?;

        // factions: FactionInitializer[amount_of_factions]
        let mut factions = Vec::with_capacity(amount_of_factions as usize);
        for i in 0..amount_of_factions {
            factions.push(FactionInitializer::tokio_read(r).await?);
        }

        Ok(Self {
            factions,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // amount_of_factions: u32
        w.write_all(&(self.factions.len() as u32).to_le_bytes()).await?;

        // factions: FactionInitializer[amount_of_factions]
        for i in self.factions.iter() {
            i.tokio_write(w).await?;
        }

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // amount_of_factions: u32
        let amount_of_factions = crate::util::astd_read_u32_le(r).await?;

        // factions: FactionInitializer[amount_of_factions]
        let mut factions = Vec::with_capacity(amount_of_factions as usize);
        for i in 0..amount_of_factions {
            factions.push(FactionInitializer::astd_read(r).await?);
        }

        Ok(Self {
            factions,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // amount_of_factions: u32
        w.write_all(&(self.factions.len() as u32).to_le_bytes()).await?;

        // factions: FactionInitializer[amount_of_factions]
        for i in self.factions.iter() {
            i.astd_write(w).await?;
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

