use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{PetCommandState, PetCommandStateError};
use crate::world::v1::v12::{PetReactState, PetReactStateError};
use crate::world::v1::v12::PetSpellCooldown;
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_PET_SPELLS {
    pub pet: Guid,
    pub unknown1: u32,
    pub react: PetReactState,
    pub command: PetCommandState,
    pub unknown2: u16,
    pub action_bars: [u32; 10],
    pub spells: Vec<u32>,
    pub cooldowns: Vec<PetSpellCooldown>,
}

impl ServerMessageWrite for SMSG_PET_SPELLS {}

impl SMSG_PET_SPELLS {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(4154);
        // pet: Guid
        w.write_all(&self.pet.guid().to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // react: PetReactState
        w.write_all(&(self.react.as_int() as u8).to_le_bytes())?;

        // command: PetCommandState
        w.write_all(&(self.command.as_int() as u8).to_le_bytes())?;

        // unknown2: u16
        w.write_all(&self.unknown2.to_le_bytes())?;

        // action_bars: u32[10]
        for i in self.action_bars.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // amount_of_spells: u8
        w.write_all(&(self.spells.len() as u8).to_le_bytes())?;

        // spells: u32[amount_of_spells]
        for i in self.spells.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // amount_of_cooldowns: u8
        w.write_all(&(self.cooldowns.len() as u8).to_le_bytes())?;

        // cooldowns: PetSpellCooldown[amount_of_cooldowns]
        for i in self.cooldowns.iter() {
            w.write_all(&(i.as_bytes()?))?;
        }

        Ok(w)
    }
}

impl MessageBody for SMSG_PET_SPELLS {
    const OPCODE: u16 = 0x0179;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_PET_SPELLSError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // pet: Guid
        let pet = Guid::read(r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        // react: PetReactState
        let react: PetReactState = crate::util::read_u8_le(r)?.try_into()?;

        // command: PetCommandState
        let command: PetCommandState = crate::util::read_u8_le(r)?.try_into()?;

        // unknown2: u16
        let unknown2 = crate::util::read_u16_le(r)?;

        // action_bars: u32[10]
        let mut action_bars = Vec::with_capacity(10 as usize);
        for i in 0..10 {
            action_bars.push(crate::util::read_u32_le(r)?);
        }
        let action_bars = action_bars.try_into().unwrap();

        // amount_of_spells: u8
        let amount_of_spells = crate::util::read_u8_le(r)?;

        // spells: u32[amount_of_spells]
        let mut spells = Vec::with_capacity(amount_of_spells as usize);
        for i in 0..amount_of_spells {
            spells.push(crate::util::read_u32_le(r)?);
        }

        // amount_of_cooldowns: u8
        let amount_of_cooldowns = crate::util::read_u8_le(r)?;

        // cooldowns: PetSpellCooldown[amount_of_cooldowns]
        let mut cooldowns = Vec::with_capacity(amount_of_cooldowns as usize);
        for i in 0..amount_of_cooldowns {
            cooldowns.push(PetSpellCooldown::read(r)?);
        }

        Ok(Self {
            pet,
            unknown1,
            react,
            command,
            unknown2,
            action_bars,
            spells,
            cooldowns,
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
            // pet: Guid
            let pet = Guid::tokio_read(r).await?;

            // unknown1: u32
            let unknown1 = crate::util::tokio_read_u32_le(r).await?;

            // react: PetReactState
            let react: PetReactState = crate::util::tokio_read_u8_le(r).await?.try_into()?;

            // command: PetCommandState
            let command: PetCommandState = crate::util::tokio_read_u8_le(r).await?.try_into()?;

            // unknown2: u16
            let unknown2 = crate::util::tokio_read_u16_le(r).await?;

            // action_bars: u32[10]
            let mut action_bars = Vec::with_capacity(10 as usize);
            for i in 0..10 {
                action_bars.push(crate::util::tokio_read_u32_le(r).await?);
            }
            let action_bars = action_bars.try_into().unwrap();

            // amount_of_spells: u8
            let amount_of_spells = crate::util::tokio_read_u8_le(r).await?;

            // spells: u32[amount_of_spells]
            let mut spells = Vec::with_capacity(amount_of_spells as usize);
            for i in 0..amount_of_spells {
                spells.push(crate::util::tokio_read_u32_le(r).await?);
            }

            // amount_of_cooldowns: u8
            let amount_of_cooldowns = crate::util::tokio_read_u8_le(r).await?;

            // cooldowns: PetSpellCooldown[amount_of_cooldowns]
            let mut cooldowns = Vec::with_capacity(amount_of_cooldowns as usize);
            for i in 0..amount_of_cooldowns {
                cooldowns.push(PetSpellCooldown::tokio_read(r).await?);
            }

            Ok(Self {
                pet,
                unknown1,
                react,
                command,
                unknown2,
                action_bars,
                spells,
                cooldowns,
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
            // pet: Guid
            let pet = Guid::astd_read(r).await?;

            // unknown1: u32
            let unknown1 = crate::util::astd_read_u32_le(r).await?;

            // react: PetReactState
            let react: PetReactState = crate::util::astd_read_u8_le(r).await?.try_into()?;

            // command: PetCommandState
            let command: PetCommandState = crate::util::astd_read_u8_le(r).await?.try_into()?;

            // unknown2: u16
            let unknown2 = crate::util::astd_read_u16_le(r).await?;

            // action_bars: u32[10]
            let mut action_bars = Vec::with_capacity(10 as usize);
            for i in 0..10 {
                action_bars.push(crate::util::astd_read_u32_le(r).await?);
            }
            let action_bars = action_bars.try_into().unwrap();

            // amount_of_spells: u8
            let amount_of_spells = crate::util::astd_read_u8_le(r).await?;

            // spells: u32[amount_of_spells]
            let mut spells = Vec::with_capacity(amount_of_spells as usize);
            for i in 0..amount_of_spells {
                spells.push(crate::util::astd_read_u32_le(r).await?);
            }

            // amount_of_cooldowns: u8
            let amount_of_cooldowns = crate::util::astd_read_u8_le(r).await?;

            // cooldowns: PetSpellCooldown[amount_of_cooldowns]
            let mut cooldowns = Vec::with_capacity(amount_of_cooldowns as usize);
            for i in 0..amount_of_cooldowns {
                cooldowns.push(PetSpellCooldown::astd_read(r).await?);
            }

            Ok(Self {
                pet,
                unknown1,
                react,
                command,
                unknown2,
                action_bars,
                spells,
                cooldowns,
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

impl SMSG_PET_SPELLS {
    pub fn size(&self) -> usize {
        0
        + 8 // pet: Guid
        + 4 // unknown1: u32
        + 1 // react: PetReactState
        + 1 // command: PetCommandState
        + 2 // unknown2: u16
        + 10 * core::mem::size_of::<u32>() // action_bars: u32[10]
        + 1 // amount_of_spells: u8
        + self.spells.len() * core::mem::size_of::<u32>() // spells: u32[amount_of_spells]
        + 1 // amount_of_cooldowns: u8
        + self.cooldowns.len() * 12 // cooldowns: PetSpellCooldown[amount_of_cooldowns]
    }
}

#[derive(Debug)]
pub enum SMSG_PET_SPELLSError {
    Io(std::io::Error),
    PetCommandState(PetCommandStateError),
    PetReactState(PetReactStateError),
}

impl std::error::Error for SMSG_PET_SPELLSError {}
impl std::fmt::Display for SMSG_PET_SPELLSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::PetCommandState(i) => i.fmt(f),
            Self::PetReactState(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_PET_SPELLSError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<PetCommandStateError> for SMSG_PET_SPELLSError {
    fn from(e: PetCommandStateError) -> Self {
        Self::PetCommandState(e)
    }
}

impl From<PetReactStateError> for SMSG_PET_SPELLSError {
    fn from(e: PetReactStateError) -> Self {
        Self::PetReactState(e)
    }
}

