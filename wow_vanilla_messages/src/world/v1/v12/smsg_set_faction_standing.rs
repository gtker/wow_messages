use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::Faction;
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
pub struct SMSG_SET_FACTION_STANDING {
    pub factions: Vec<Faction>,
}

impl ServerMessageWrite for SMSG_SET_FACTION_STANDING {}

impl MessageBody for SMSG_SET_FACTION_STANDING {
    const OPCODE: u16 = 0x0124;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
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

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // amount_of_factions: u32
        w.write_all(&(self.factions.len() as u32).to_le_bytes())?;

        // factions: Faction[amount_of_factions]
        for i in self.factions.iter() {
            i.write(w)?;
        }

        Ok(())
    }

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
            // amount_of_factions: u32
            let amount_of_factions = crate::util::tokio_read_u32_le(r).await?;

            // factions: Faction[amount_of_factions]
            let mut factions = Vec::with_capacity(amount_of_factions as usize);
            for i in 0..amount_of_factions {
                factions.push(Faction::tokio_read(r).await?);
            }

            Ok(Self {
                factions,
            })
        })
    }

    fn tokio_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // amount_of_factions: u32
            w.write_all(&(self.factions.len() as u32).to_le_bytes()).await?;

            // factions: Faction[amount_of_factions]
            for i in self.factions.iter() {
                i.tokio_write(w).await?;
            }

            Ok(())
        })
    }

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
            // amount_of_factions: u32
            let amount_of_factions = crate::util::astd_read_u32_le(r).await?;

            // factions: Faction[amount_of_factions]
            let mut factions = Vec::with_capacity(amount_of_factions as usize);
            for i in 0..amount_of_factions {
                factions.push(Faction::astd_read(r).await?);
            }

            Ok(Self {
                factions,
            })
        })
    }

    fn astd_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // amount_of_factions: u32
            w.write_all(&(self.factions.len() as u32).to_le_bytes()).await?;

            // factions: Faction[amount_of_factions]
            for i in self.factions.iter() {
                i.astd_write(w).await?;
            }

            Ok(())
        })
    }

}

impl VariableSized for SMSG_SET_FACTION_STANDING {
    fn size(&self) -> usize {
        0
        + 4 // amount_of_factions: u32
        + self.factions.iter().fold(0, |acc, x| acc + Faction::size()) // factions: Faction[amount_of_factions]
    }
}

impl MaximumPossibleSized for SMSG_SET_FACTION_STANDING {
    fn maximum_possible_size() -> usize {
        65535 // Capped at u16::MAX due to size field.
    }
}

