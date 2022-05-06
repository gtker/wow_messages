use std::convert::{TryFrom, TryInto};
use crate::Guid;
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
pub struct SMSG_LOOT_MASTER_LIST {
    pub guids: Vec<Guid>,
}

impl ServerMessageWrite for SMSG_LOOT_MASTER_LIST {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_LOOT_MASTER_LIST {
    const OPCODE: u16 = 0x02a4;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // amount_of_players: u8
        let amount_of_players = crate::util::read_u8_le(r)?;

        // guids: Guid[amount_of_players]
        let mut guids = Vec::with_capacity(amount_of_players as usize);
        for i in 0..amount_of_players {
            guids.push(Guid::read(r)?);
        }

        Ok(Self {
            guids,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // amount_of_players: u8
        w.write_all(&(self.guids.len() as u8).to_le_bytes())?;

        // guids: Guid[amount_of_players]
        for i in self.guids.iter() {
            i.write(w)?;
        }

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // amount_of_players: u8
        let amount_of_players = crate::util::tokio_read_u8_le(r).await?;

        // guids: Guid[amount_of_players]
        let mut guids = Vec::with_capacity(amount_of_players as usize);
        for i in 0..amount_of_players {
            guids.push(Guid::tokio_read(r).await?);
        }

        Ok(Self {
            guids,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // amount_of_players: u8
        w.write_all(&(self.guids.len() as u8).to_le_bytes()).await?;

        // guids: Guid[amount_of_players]
        for i in self.guids.iter() {
            i.tokio_write(w).await?;
        }

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // amount_of_players: u8
        let amount_of_players = crate::util::astd_read_u8_le(r).await?;

        // guids: Guid[amount_of_players]
        let mut guids = Vec::with_capacity(amount_of_players as usize);
        for i in 0..amount_of_players {
            guids.push(Guid::astd_read(r).await?);
        }

        Ok(Self {
            guids,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // amount_of_players: u8
        w.write_all(&(self.guids.len() as u8).to_le_bytes()).await?;

        // guids: Guid[amount_of_players]
        for i in self.guids.iter() {
            i.astd_write(w).await?;
        }

        Ok(())
    }

}

impl VariableSized for SMSG_LOOT_MASTER_LIST {
    fn size(&self) -> usize {
        0
        + 1 // amount_of_players: u8
        + self.guids.iter().fold(0, |acc, _| acc + 8) // guids: Guid[amount_of_players]
    }
}

impl MaximumPossibleSized for SMSG_LOOT_MASTER_LIST {
    fn maximum_possible_size() -> usize {
        0
        + 1 // amount_of_players: u8
        + 2048 // guids: Guid[amount_of_players]
    }
}

