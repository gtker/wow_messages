use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::PetitionShowlist;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_PETITION_SHOWLIST {
    pub npc: Guid,
    pub petitions: Vec<PetitionShowlist>,
}

impl SMSG_PETITION_SHOWLIST {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // npc: Guid
        w.write_all(&self.npc.guid().to_le_bytes())?;

        // amount_of_petitions: u8
        w.write_all(&(self.petitions.len() as u8).to_le_bytes())?;

        // petitions: PetitionShowlist[amount_of_petitions]
        for i in self.petitions.iter() {
            w.write_all(&(i.as_bytes()?))?;
        }

        Ok(w)
    }
}

impl ServerMessage for SMSG_PETITION_SHOWLIST {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // npc: Guid
        w.write_all(&self.npc.guid().to_le_bytes())?;

        // amount_of_petitions: u8
        w.write_all(&(self.petitions.len() as u8).to_le_bytes())?;

        // petitions: PetitionShowlist[amount_of_petitions]
        for i in self.petitions.iter() {
            w.write_all(&(i.as_bytes()?))?;
        }

        Ok(())
    }
    const OPCODE: u16 = 0x01bc;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // npc: Guid
        let npc = Guid::read(r)?;

        // amount_of_petitions: u8
        let amount_of_petitions = crate::util::read_u8_le(r)?;

        // petitions: PetitionShowlist[amount_of_petitions]
        let mut petitions = Vec::with_capacity(amount_of_petitions as usize);
        for i in 0..amount_of_petitions {
            petitions.push(PetitionShowlist::read(r)?);
        }

        Ok(Self {
            npc,
            petitions,
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

            // amount_of_petitions: u8
            let amount_of_petitions = crate::util::tokio_read_u8_le(r).await?;

            // petitions: PetitionShowlist[amount_of_petitions]
            let mut petitions = Vec::with_capacity(amount_of_petitions as usize);
            for i in 0..amount_of_petitions {
                petitions.push(PetitionShowlist::tokio_read(r).await?);
            }

            Ok(Self {
                npc,
                petitions,
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

            // amount_of_petitions: u8
            let amount_of_petitions = crate::util::astd_read_u8_le(r).await?;

            // petitions: PetitionShowlist[amount_of_petitions]
            let mut petitions = Vec::with_capacity(amount_of_petitions as usize);
            for i in 0..amount_of_petitions {
                petitions.push(PetitionShowlist::astd_read(r).await?);
            }

            Ok(Self {
                npc,
                petitions,
            })
        })
    }

}

impl SMSG_PETITION_SHOWLIST {
    pub fn size(&self) -> usize {
        0
        + 8 // npc: Guid
        + 1 // amount_of_petitions: u8
        + self.petitions.len() * 24 // petitions: PetitionShowlist[amount_of_petitions]
    }
}

