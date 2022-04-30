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
pub struct SMSG_SPELL_UPDATE_CHAIN_TARGETS {
    pub caster: Guid,
    pub spell: u32,
    pub targets: Vec<Guid>,
}

impl ServerMessageWrite for SMSG_SPELL_UPDATE_CHAIN_TARGETS {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_SPELL_UPDATE_CHAIN_TARGETS {
    const OPCODE: u16 = 0x0330;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

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
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
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
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
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
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
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
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
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
    }
}

impl VariableSized for SMSG_SPELL_UPDATE_CHAIN_TARGETS {
    fn size(&self) -> usize {
        8 // caster: Guid
        + 4 // spell: u32
        + 4 // amount_of_targets: u32
        + self.targets.iter().fold(0, |acc, _| acc + 8) // targets: Guid[amount_of_targets]
    }
}

impl MaximumPossibleSized for SMSG_SPELL_UPDATE_CHAIN_TARGETS {
    fn maximum_possible_size() -> usize {
        8 // caster: Guid
        + 4 // spell: u32
        + 4 // amount_of_targets: u32
        + 4294967295 * 8 // targets: Guid[amount_of_targets]
    }
}

