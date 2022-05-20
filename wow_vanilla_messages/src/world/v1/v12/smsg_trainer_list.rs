use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{TrainerSpell, TrainerSpellError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_TRAINER_LIST {
    pub guid: Guid,
    pub trainer_type: u32,
    pub spells: Vec<TrainerSpell>,
    pub greeting: String,
}

impl ServerMessageWrite for SMSG_TRAINER_LIST {}

impl SMSG_TRAINER_LIST {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(163208757520);
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // trainer_type: u32
        w.write_all(&self.trainer_type.to_le_bytes())?;

        // amount_of_spells: u32
        w.write_all(&(self.spells.len() as u32).to_le_bytes())?;

        // spells: TrainerSpell[amount_of_spells]
        for i in self.spells.iter() {
            w.write_all(&(i.as_bytes()?))?;
        }

        // greeting: CString
        w.write_all(self.greeting.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(w)
    }
}

impl MessageBody for SMSG_TRAINER_LIST {
    const OPCODE: u16 = 0x01b1;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_TRAINER_LISTError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // trainer_type: u32
        let trainer_type = crate::util::read_u32_le(r)?;

        // amount_of_spells: u32
        let amount_of_spells = crate::util::read_u32_le(r)?;

        // spells: TrainerSpell[amount_of_spells]
        let mut spells = Vec::with_capacity(amount_of_spells as usize);
        for i in 0..amount_of_spells {
            spells.push(TrainerSpell::read(r)?);
        }

        // greeting: CString
        let greeting = crate::util::read_c_string_to_vec(r)?;
        let greeting = String::from_utf8(greeting)?;

        Ok(Self {
            guid,
            trainer_type,
            spells,
            greeting,
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
            // guid: Guid
            let guid = Guid::tokio_read(r).await?;

            // trainer_type: u32
            let trainer_type = crate::util::tokio_read_u32_le(r).await?;

            // amount_of_spells: u32
            let amount_of_spells = crate::util::tokio_read_u32_le(r).await?;

            // spells: TrainerSpell[amount_of_spells]
            let mut spells = Vec::with_capacity(amount_of_spells as usize);
            for i in 0..amount_of_spells {
                spells.push(TrainerSpell::tokio_read(r).await?);
            }

            // greeting: CString
            let greeting = crate::util::tokio_read_c_string_to_vec(r).await?;
            let greeting = String::from_utf8(greeting)?;

            Ok(Self {
                guid,
                trainer_type,
                spells,
                greeting,
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
            // guid: Guid
            let guid = Guid::astd_read(r).await?;

            // trainer_type: u32
            let trainer_type = crate::util::astd_read_u32_le(r).await?;

            // amount_of_spells: u32
            let amount_of_spells = crate::util::astd_read_u32_le(r).await?;

            // spells: TrainerSpell[amount_of_spells]
            let mut spells = Vec::with_capacity(amount_of_spells as usize);
            for i in 0..amount_of_spells {
                spells.push(TrainerSpell::astd_read(r).await?);
            }

            // greeting: CString
            let greeting = crate::util::astd_read_c_string_to_vec(r).await?;
            let greeting = String::from_utf8(greeting)?;

            Ok(Self {
                guid,
                trainer_type,
                spells,
                greeting,
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

impl SMSG_TRAINER_LIST {
    pub fn size(&self) -> usize {
        0
        + 8 // guid: Guid
        + 4 // trainer_type: u32
        + 4 // amount_of_spells: u32
        + self.spells.len() * 38 // spells: TrainerSpell[amount_of_spells]
        + self.greeting.len() + 1 // greeting: CString
    }
}

#[derive(Debug)]
pub enum SMSG_TRAINER_LISTError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
    TrainerSpell(TrainerSpellError),
}

impl std::error::Error for SMSG_TRAINER_LISTError {}
impl std::fmt::Display for SMSG_TRAINER_LISTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
            Self::TrainerSpell(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_TRAINER_LISTError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_TRAINER_LISTError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

impl From<TrainerSpellError> for SMSG_TRAINER_LISTError {
    fn from(e: TrainerSpellError) -> Self {
        Self::TrainerSpell(e)
    }
}

