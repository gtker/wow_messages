use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_SPELL_UPDATE_CHAIN_TARGETS {
    pub caster: Guid,
    pub spell: u32,
    pub targets: Vec<Guid>,
}

impl ServerMessageWrite for SMSG_SPELL_UPDATE_CHAIN_TARGETS {}

impl MessageBody for SMSG_SPELL_UPDATE_CHAIN_TARGETS {
    const OPCODE: u16 = 0x0330;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // caster: Guid
        let caster = Guid::read(r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        // amount_of_targets: u32
        let amount_of_targets = crate::util::read_u32_le(r)?;

        // targets: Guid[amount_of_targets]
        let mut targets = Vec::with_capacity(amount_of_targets as usize);
        for i in 0..amount_of_targets {
            targets.push(Guid::read(r)?);
        }

        Ok(Self {
            caster,
            spell,
            targets,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // caster: Guid
        self.caster.write(w)?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // amount_of_targets: u32
        w.write_all(&(self.targets.len() as u32).to_le_bytes())?;

        // targets: Guid[amount_of_targets]
        for i in self.targets.iter() {
            i.write(w)?;
        }

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
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
            // caster: Guid
            let caster = Guid::tokio_read(r).await?;

            // spell: u32
            let spell = crate::util::tokio_read_u32_le(r).await?;

            // amount_of_targets: u32
            let amount_of_targets = crate::util::tokio_read_u32_le(r).await?;

            // targets: Guid[amount_of_targets]
            let mut targets = Vec::with_capacity(amount_of_targets as usize);
            for i in 0..amount_of_targets {
                targets.push(Guid::tokio_read(r).await?);
            }

            Ok(Self {
                caster,
                spell,
                targets,
            })
        })
    }

    #[cfg(feature = "async_tokio")]
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
            // caster: Guid
            self.caster.tokio_write(w).await?;

            // spell: u32
            w.write_all(&self.spell.to_le_bytes()).await?;

            // amount_of_targets: u32
            w.write_all(&(self.targets.len() as u32).to_le_bytes()).await?;

            // targets: Guid[amount_of_targets]
            for i in self.targets.iter() {
                i.tokio_write(w).await?;
            }

            Ok(())
        })
    }

    #[cfg(feature = "async_std")]
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
            // caster: Guid
            let caster = Guid::astd_read(r).await?;

            // spell: u32
            let spell = crate::util::astd_read_u32_le(r).await?;

            // amount_of_targets: u32
            let amount_of_targets = crate::util::astd_read_u32_le(r).await?;

            // targets: Guid[amount_of_targets]
            let mut targets = Vec::with_capacity(amount_of_targets as usize);
            for i in 0..amount_of_targets {
                targets.push(Guid::astd_read(r).await?);
            }

            Ok(Self {
                caster,
                spell,
                targets,
            })
        })
    }

    #[cfg(feature = "async_std")]
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
            // caster: Guid
            self.caster.astd_write(w).await?;

            // spell: u32
            w.write_all(&self.spell.to_le_bytes()).await?;

            // amount_of_targets: u32
            w.write_all(&(self.targets.len() as u32).to_le_bytes()).await?;

            // targets: Guid[amount_of_targets]
            for i in self.targets.iter() {
                i.astd_write(w).await?;
            }

            Ok(())
        })
    }

}

impl VariableSized for SMSG_SPELL_UPDATE_CHAIN_TARGETS {
    fn size(&self) -> usize {
        0
        + 8 // caster: Guid
        + 4 // spell: u32
        + 4 // amount_of_targets: u32
        + self.targets.iter().fold(0, |acc, _| acc + 8) // targets: Guid[amount_of_targets]
    }
}

impl MaximumPossibleSized for SMSG_SPELL_UPDATE_CHAIN_TARGETS {
    fn maximum_possible_size() -> usize {
        65535 // Capped at u16::MAX due to size field.
    }
}

