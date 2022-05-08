use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::CooldownSpell;
use crate::world::v1::v12::InitialSpell;
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_INITIAL_SPELLS {
    pub unknown1: u8,
    pub initial_spells: Vec<InitialSpell>,
    pub cooldowns: Vec<CooldownSpell>,
}

impl ServerMessageWrite for SMSG_INITIAL_SPELLS {}

impl MessageBody for SMSG_INITIAL_SPELLS {
    const OPCODE: u16 = 0x012a;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(r)?;

        // spell_count: u16
        let spell_count = crate::util::read_u16_le(r)?;

        // initial_spells: InitialSpell[spell_count]
        let mut initial_spells = Vec::with_capacity(spell_count as usize);
        for i in 0..spell_count {
            initial_spells.push(InitialSpell::read(r)?);
        }

        // cooldown_count: u16
        let cooldown_count = crate::util::read_u16_le(r)?;

        // cooldowns: CooldownSpell[cooldown_count]
        let mut cooldowns = Vec::with_capacity(cooldown_count as usize);
        for i in 0..cooldown_count {
            cooldowns.push(CooldownSpell::read(r)?);
        }

        Ok(Self {
            unknown1,
            initial_spells,
            cooldowns,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        // spell_count: u16
        w.write_all(&(self.initial_spells.len() as u16).to_le_bytes())?;

        // initial_spells: InitialSpell[spell_count]
        for i in self.initial_spells.iter() {
            i.write(w)?;
        }

        // cooldown_count: u16
        w.write_all(&(self.cooldowns.len() as u16).to_le_bytes())?;

        // cooldowns: CooldownSpell[cooldown_count]
        for i in self.cooldowns.iter() {
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
            // unknown1: u8
            let unknown1 = crate::util::tokio_read_u8_le(r).await?;

            // spell_count: u16
            let spell_count = crate::util::tokio_read_u16_le(r).await?;

            // initial_spells: InitialSpell[spell_count]
            let mut initial_spells = Vec::with_capacity(spell_count as usize);
            for i in 0..spell_count {
                initial_spells.push(InitialSpell::tokio_read(r).await?);
            }

            // cooldown_count: u16
            let cooldown_count = crate::util::tokio_read_u16_le(r).await?;

            // cooldowns: CooldownSpell[cooldown_count]
            let mut cooldowns = Vec::with_capacity(cooldown_count as usize);
            for i in 0..cooldown_count {
                cooldowns.push(CooldownSpell::tokio_read(r).await?);
            }

            Ok(Self {
                unknown1,
                initial_spells,
                cooldowns,
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
            // unknown1: u8
            w.write_all(&self.unknown1.to_le_bytes()).await?;

            // spell_count: u16
            w.write_all(&(self.initial_spells.len() as u16).to_le_bytes()).await?;

            // initial_spells: InitialSpell[spell_count]
            for i in self.initial_spells.iter() {
                i.tokio_write(w).await?;
            }

            // cooldown_count: u16
            w.write_all(&(self.cooldowns.len() as u16).to_le_bytes()).await?;

            // cooldowns: CooldownSpell[cooldown_count]
            for i in self.cooldowns.iter() {
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
            // unknown1: u8
            let unknown1 = crate::util::astd_read_u8_le(r).await?;

            // spell_count: u16
            let spell_count = crate::util::astd_read_u16_le(r).await?;

            // initial_spells: InitialSpell[spell_count]
            let mut initial_spells = Vec::with_capacity(spell_count as usize);
            for i in 0..spell_count {
                initial_spells.push(InitialSpell::astd_read(r).await?);
            }

            // cooldown_count: u16
            let cooldown_count = crate::util::astd_read_u16_le(r).await?;

            // cooldowns: CooldownSpell[cooldown_count]
            let mut cooldowns = Vec::with_capacity(cooldown_count as usize);
            for i in 0..cooldown_count {
                cooldowns.push(CooldownSpell::astd_read(r).await?);
            }

            Ok(Self {
                unknown1,
                initial_spells,
                cooldowns,
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
            // unknown1: u8
            w.write_all(&self.unknown1.to_le_bytes()).await?;

            // spell_count: u16
            w.write_all(&(self.initial_spells.len() as u16).to_le_bytes()).await?;

            // initial_spells: InitialSpell[spell_count]
            for i in self.initial_spells.iter() {
                i.astd_write(w).await?;
            }

            // cooldown_count: u16
            w.write_all(&(self.cooldowns.len() as u16).to_le_bytes()).await?;

            // cooldowns: CooldownSpell[cooldown_count]
            for i in self.cooldowns.iter() {
                i.astd_write(w).await?;
            }

            Ok(())
        })
    }

}

impl VariableSized for SMSG_INITIAL_SPELLS {
    fn size(&self) -> usize {
        0
        + 1 // unknown1: u8
        + 2 // spell_count: u16
        + self.initial_spells.iter().fold(0, |acc, x| acc + InitialSpell::size()) // initial_spells: InitialSpell[spell_count]
        + 2 // cooldown_count: u16
        + self.cooldowns.iter().fold(0, |acc, x| acc + CooldownSpell::size()) // cooldowns: CooldownSpell[cooldown_count]
    }
}

impl MaximumPossibleSized for SMSG_INITIAL_SPELLS {
    fn maximum_possible_size() -> usize {
        65535 // Capped at u16::MAX due to size field.
    }
}

