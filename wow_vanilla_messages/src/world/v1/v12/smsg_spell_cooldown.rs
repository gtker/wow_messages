use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::SpellCooldownStatus;
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
pub struct SMSG_SPELL_COOLDOWN {
    pub guid: Guid,
    pub cooldowns: Vec<SpellCooldownStatus>,
}

impl ServerMessageWrite for SMSG_SPELL_COOLDOWN {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_SPELL_COOLDOWN {
    const OPCODE: u16 = 0x0134;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // cooldowns: SpellCooldownStatus[-]
        let mut current_size = {
            8 // guid: Guid
        };
        let mut cooldowns = Vec::with_capacity(body_size as usize - current_size);
        while current_size < (body_size as usize) {
            cooldowns.push(SpellCooldownStatus::read(r)?);
            current_size += 1;
        }

        Ok(Self {
            guid,
            cooldowns,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // cooldowns: SpellCooldownStatus[-]
        for i in self.cooldowns.iter() {
            i.write(w)?;
        }

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::tokio_read(r).await?;

        // cooldowns: SpellCooldownStatus[-]
        let mut current_size = {
            8 // guid: Guid
        };
        let mut cooldowns = Vec::with_capacity(body_size as usize - current_size);
        while current_size < (body_size as usize) {
            cooldowns.push(SpellCooldownStatus::tokio_read(r).await?);
            current_size += 1;
        }

        Ok(Self {
            guid,
            cooldowns,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.tokio_write(w).await?;

        // cooldowns: SpellCooldownStatus[-]
        for i in self.cooldowns.iter() {
            i.tokio_write(w).await?;
        }

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::astd_read(r).await?;

        // cooldowns: SpellCooldownStatus[-]
        let mut current_size = {
            8 // guid: Guid
        };
        let mut cooldowns = Vec::with_capacity(body_size as usize - current_size);
        while current_size < (body_size as usize) {
            cooldowns.push(SpellCooldownStatus::astd_read(r).await?);
            current_size += 1;
        }

        Ok(Self {
            guid,
            cooldowns,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.astd_write(w).await?;

        // cooldowns: SpellCooldownStatus[-]
        for i in self.cooldowns.iter() {
            i.astd_write(w).await?;
        }

        Ok(())
    }
}

impl VariableSized for SMSG_SPELL_COOLDOWN {
    fn size(&self) -> usize {
        8 // guid: Guid
        + self.cooldowns.iter().fold(0, |acc, x| acc + SpellCooldownStatus::size()) // cooldowns: SpellCooldownStatus[-]
    }
}

impl MaximumPossibleSized for SMSG_SPELL_COOLDOWN {
    fn maximum_possible_size() -> usize {
        8 // guid: Guid
        + 65536 // cooldowns: SpellCooldownStatus[-]
    }
}

