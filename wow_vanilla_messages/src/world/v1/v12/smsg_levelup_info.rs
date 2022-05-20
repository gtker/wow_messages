use std::convert::{TryFrom, TryInto};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_LEVELUP_INFO {
    pub new_level: u32,
    pub health: u32,
    pub mana: u32,
    pub rage: u32,
    pub focus: u32,
    pub energy: u32,
    pub happiness: u32,
    pub strength: u32,
    pub agility: u32,
    pub stamina: u32,
    pub intellect: u32,
    pub spirit: u32,
}

impl ServerMessageWrite for SMSG_LEVELUP_INFO {}

impl SMSG_LEVELUP_INFO {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(8000);
        // new_level: u32
        w.write_all(&self.new_level.to_le_bytes())?;

        // health: u32
        w.write_all(&self.health.to_le_bytes())?;

        // mana: u32
        w.write_all(&self.mana.to_le_bytes())?;

        // rage: u32
        w.write_all(&self.rage.to_le_bytes())?;

        // focus: u32
        w.write_all(&self.focus.to_le_bytes())?;

        // energy: u32
        w.write_all(&self.energy.to_le_bytes())?;

        // happiness: u32
        w.write_all(&self.happiness.to_le_bytes())?;

        // strength: u32
        w.write_all(&self.strength.to_le_bytes())?;

        // agility: u32
        w.write_all(&self.agility.to_le_bytes())?;

        // stamina: u32
        w.write_all(&self.stamina.to_le_bytes())?;

        // intellect: u32
        w.write_all(&self.intellect.to_le_bytes())?;

        // spirit: u32
        w.write_all(&self.spirit.to_le_bytes())?;

        Ok(w)
    }
}

impl MessageBody for SMSG_LEVELUP_INFO {
    const OPCODE: u16 = 0x01d4;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // new_level: u32
        let new_level = crate::util::read_u32_le(r)?;

        // health: u32
        let health = crate::util::read_u32_le(r)?;

        // mana: u32
        let mana = crate::util::read_u32_le(r)?;

        // rage: u32
        let rage = crate::util::read_u32_le(r)?;

        // focus: u32
        let focus = crate::util::read_u32_le(r)?;

        // energy: u32
        let energy = crate::util::read_u32_le(r)?;

        // happiness: u32
        let happiness = crate::util::read_u32_le(r)?;

        // strength: u32
        let strength = crate::util::read_u32_le(r)?;

        // agility: u32
        let agility = crate::util::read_u32_le(r)?;

        // stamina: u32
        let stamina = crate::util::read_u32_le(r)?;

        // intellect: u32
        let intellect = crate::util::read_u32_le(r)?;

        // spirit: u32
        let spirit = crate::util::read_u32_le(r)?;

        Ok(Self {
            new_level,
            health,
            mana,
            rage,
            focus,
            energy,
            happiness,
            strength,
            agility,
            stamina,
            intellect,
            spirit,
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
            // new_level: u32
            let new_level = crate::util::tokio_read_u32_le(r).await?;

            // health: u32
            let health = crate::util::tokio_read_u32_le(r).await?;

            // mana: u32
            let mana = crate::util::tokio_read_u32_le(r).await?;

            // rage: u32
            let rage = crate::util::tokio_read_u32_le(r).await?;

            // focus: u32
            let focus = crate::util::tokio_read_u32_le(r).await?;

            // energy: u32
            let energy = crate::util::tokio_read_u32_le(r).await?;

            // happiness: u32
            let happiness = crate::util::tokio_read_u32_le(r).await?;

            // strength: u32
            let strength = crate::util::tokio_read_u32_le(r).await?;

            // agility: u32
            let agility = crate::util::tokio_read_u32_le(r).await?;

            // stamina: u32
            let stamina = crate::util::tokio_read_u32_le(r).await?;

            // intellect: u32
            let intellect = crate::util::tokio_read_u32_le(r).await?;

            // spirit: u32
            let spirit = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                new_level,
                health,
                mana,
                rage,
                focus,
                energy,
                happiness,
                strength,
                agility,
                stamina,
                intellect,
                spirit,
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
            // new_level: u32
            let new_level = crate::util::astd_read_u32_le(r).await?;

            // health: u32
            let health = crate::util::astd_read_u32_le(r).await?;

            // mana: u32
            let mana = crate::util::astd_read_u32_le(r).await?;

            // rage: u32
            let rage = crate::util::astd_read_u32_le(r).await?;

            // focus: u32
            let focus = crate::util::astd_read_u32_le(r).await?;

            // energy: u32
            let energy = crate::util::astd_read_u32_le(r).await?;

            // happiness: u32
            let happiness = crate::util::astd_read_u32_le(r).await?;

            // strength: u32
            let strength = crate::util::astd_read_u32_le(r).await?;

            // agility: u32
            let agility = crate::util::astd_read_u32_le(r).await?;

            // stamina: u32
            let stamina = crate::util::astd_read_u32_le(r).await?;

            // intellect: u32
            let intellect = crate::util::astd_read_u32_le(r).await?;

            // spirit: u32
            let spirit = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                new_level,
                health,
                mana,
                rage,
                focus,
                energy,
                happiness,
                strength,
                agility,
                stamina,
                intellect,
                spirit,
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

impl SMSG_LEVELUP_INFO {
    pub(crate) fn size() -> usize {
        0
        + 4 // new_level: u32
        + 4 // health: u32
        + 4 // mana: u32
        + 4 // rage: u32
        + 4 // focus: u32
        + 4 // energy: u32
        + 4 // happiness: u32
        + 4 // strength: u32
        + 4 // agility: u32
        + 4 // stamina: u32
        + 4 // intellect: u32
        + 4 // spirit: u32
    }
}

