use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::ForcedReaction;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_SET_FORCED_REACTIONS {
    pub reactions: Vec<ForcedReaction>,
}

impl SMSG_SET_FORCED_REACTIONS {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // amount_of_reactions: u32
        w.write_all(&(self.reactions.len() as u32).to_le_bytes())?;

        // reactions: ForcedReaction[amount_of_reactions]
        for i in self.reactions.iter() {
            w.write_all(&(i.as_bytes()?))?;
        }

        Ok(w)
    }
}

impl ServerMessage for SMSG_SET_FORCED_REACTIONS {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // amount_of_reactions: u32
        w.write_all(&(self.reactions.len() as u32).to_le_bytes())?;

        // reactions: ForcedReaction[amount_of_reactions]
        for i in self.reactions.iter() {
            w.write_all(&(i.as_bytes()?))?;
        }

        Ok(())
    }
    const OPCODE: u16 = 0x02a5;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // amount_of_reactions: u32
        let amount_of_reactions = crate::util::read_u32_le(r)?;

        // reactions: ForcedReaction[amount_of_reactions]
        let mut reactions = Vec::with_capacity(amount_of_reactions as usize);
        for i in 0..amount_of_reactions {
            reactions.push(ForcedReaction::read(r)?);
        }

        Ok(Self {
            reactions,
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
            // amount_of_reactions: u32
            let amount_of_reactions = crate::util::tokio_read_u32_le(r).await?;

            // reactions: ForcedReaction[amount_of_reactions]
            let mut reactions = Vec::with_capacity(amount_of_reactions as usize);
            for i in 0..amount_of_reactions {
                reactions.push(ForcedReaction::tokio_read(r).await?);
            }

            Ok(Self {
                reactions,
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
            // amount_of_reactions: u32
            let amount_of_reactions = crate::util::astd_read_u32_le(r).await?;

            // reactions: ForcedReaction[amount_of_reactions]
            let mut reactions = Vec::with_capacity(amount_of_reactions as usize);
            for i in 0..amount_of_reactions {
                reactions.push(ForcedReaction::astd_read(r).await?);
            }

            Ok(Self {
                reactions,
            })
        })
    }

}

impl SMSG_SET_FORCED_REACTIONS {
    pub fn size(&self) -> usize {
        0
        + 4 // amount_of_reactions: u32
        + self.reactions.len() * 8 // reactions: ForcedReaction[amount_of_reactions]
    }
}

