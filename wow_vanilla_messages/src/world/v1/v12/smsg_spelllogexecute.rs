use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{SpellLog, SpellLogError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_SPELLLOGEXECUTE {
    pub caster: Guid,
    pub spell: u32,
    pub logs: Vec<SpellLog>,
}

impl ServerMessageWrite for SMSG_SPELLLOGEXECUTE {}

impl MessageBody for SMSG_SPELLLOGEXECUTE {
    const OPCODE: u16 = 0x024c;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_SPELLLOGEXECUTEError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // caster: PackedGuid
        let caster = Guid::read_packed(r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        // amount_of_effects: u32
        let amount_of_effects = crate::util::read_u32_le(r)?;

        // logs: SpellLog[amount_of_effects]
        let mut logs = Vec::with_capacity(amount_of_effects as usize);
        for i in 0..amount_of_effects {
            logs.push(SpellLog::read(r)?);
        }

        Ok(Self {
            caster,
            spell,
            logs,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // caster: PackedGuid
        self.caster.write_packed(w)?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // amount_of_effects: u32
        w.write_all(&(self.logs.len() as u32).to_le_bytes())?;

        // logs: SpellLog[amount_of_effects]
        for i in self.logs.iter() {
            i.write(w)?;
        }

        Ok(())
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
            // caster: PackedGuid
            let caster = Guid::tokio_read_packed(r).await?;

            // spell: u32
            let spell = crate::util::tokio_read_u32_le(r).await?;

            // amount_of_effects: u32
            let amount_of_effects = crate::util::tokio_read_u32_le(r).await?;

            // logs: SpellLog[amount_of_effects]
            let mut logs = Vec::with_capacity(amount_of_effects as usize);
            for i in 0..amount_of_effects {
                logs.push(SpellLog::tokio_read(r).await?);
            }

            Ok(Self {
                caster,
                spell,
                logs,
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
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // caster: PackedGuid
            self.caster.tokio_write_packed(w).await?;

            // spell: u32
            w.write_all(&self.spell.to_le_bytes()).await?;

            // amount_of_effects: u32
            w.write_all(&(self.logs.len() as u32).to_le_bytes()).await?;

            // logs: SpellLog[amount_of_effects]
            for i in self.logs.iter() {
                i.tokio_write(w).await?;
            }

            Ok(())
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
            // caster: PackedGuid
            let caster = Guid::astd_read_packed(r).await?;

            // spell: u32
            let spell = crate::util::astd_read_u32_le(r).await?;

            // amount_of_effects: u32
            let amount_of_effects = crate::util::astd_read_u32_le(r).await?;

            // logs: SpellLog[amount_of_effects]
            let mut logs = Vec::with_capacity(amount_of_effects as usize);
            for i in 0..amount_of_effects {
                logs.push(SpellLog::astd_read(r).await?);
            }

            Ok(Self {
                caster,
                spell,
                logs,
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
        W: 'async_trait + WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // caster: PackedGuid
            self.caster.astd_write_packed(w).await?;

            // spell: u32
            w.write_all(&self.spell.to_le_bytes()).await?;

            // amount_of_effects: u32
            w.write_all(&(self.logs.len() as u32).to_le_bytes()).await?;

            // logs: SpellLog[amount_of_effects]
            for i in self.logs.iter() {
                i.astd_write(w).await?;
            }

            Ok(())
        })
    }

}

impl SMSG_SPELLLOGEXECUTE {
    pub fn size(&self) -> usize {
        0
        + self.caster.size() // caster: Guid
        + 4 // spell: u32
        + 4 // amount_of_effects: u32
        + self.logs.iter().fold(0, |acc, x| acc + x.size()) // logs: SpellLog[amount_of_effects]
    }
}

#[derive(Debug)]
pub enum SMSG_SPELLLOGEXECUTEError {
    Io(std::io::Error),
    SpellLog(SpellLogError),
}

impl std::error::Error for SMSG_SPELLLOGEXECUTEError {}
impl std::fmt::Display for SMSG_SPELLLOGEXECUTEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::SpellLog(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_SPELLLOGEXECUTEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<SpellLogError> for SMSG_SPELLLOGEXECUTEError {
    fn from(e: SpellLogError) -> Self {
        Self::SpellLog(e)
    }
}

