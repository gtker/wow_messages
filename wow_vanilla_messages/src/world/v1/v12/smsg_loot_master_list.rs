use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_LOOT_MASTER_LIST {
    pub guids: Vec<Guid>,
}

impl SMSG_LOOT_MASTER_LIST {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // amount_of_players: u8
        w.write_all(&(self.guids.len() as u8).to_le_bytes())?;

        // guids: Guid[amount_of_players]
        for i in self.guids.iter() {
            w.write_all(&i.guid().to_le_bytes())?;
        }

        Ok(w)
    }
}

impl ServerMessage for SMSG_LOOT_MASTER_LIST {
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
        let inner = self.as_bytes()?;
        w.write_all(&inner)
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
        })
    }

    #[cfg(feature = "tokio")]
    fn tokio_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let inner = self.as_bytes()?;
            w.write_all(&inner).await
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
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let inner = self.as_bytes()?;
            w.write_all(&inner).await
        })
    }

}

impl SMSG_LOOT_MASTER_LIST {
    pub fn size(&self) -> usize {
        0
        + 1 // amount_of_players: u8
        + self.guids.iter().fold(0, |acc, _| acc + 8) // guids: Guid[amount_of_players]
    }
}

