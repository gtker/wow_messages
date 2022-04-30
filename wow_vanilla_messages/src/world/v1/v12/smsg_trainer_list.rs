use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{TrainerSpell, TrainerSpellError};
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
pub struct SMSG_TRAINER_LIST {
    pub guid: Guid,
    pub trainer_type: u32,
    pub spells: Vec<TrainerSpell>,
    pub greeting: String,
}

impl ServerMessageWrite for SMSG_TRAINER_LIST {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_TRAINER_LIST {
    const OPCODE: u16 = 0x01b1;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_TRAINER_LISTError;

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

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // trainer_type: u32
        w.write_all(&self.trainer_type.to_le_bytes())?;

        // amount_of_spells: u32
        w.write_all(&(self.spells.len() as u32).to_le_bytes())?;

        // spells: TrainerSpell[amount_of_spells]
        for i in self.spells.iter() {
            i.write(w)?;
        }

        // greeting: CString
        w.write_all(self.greeting.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
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
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.tokio_write(w).await?;

        // trainer_type: u32
        w.write_all(&self.trainer_type.to_le_bytes()).await?;

        // amount_of_spells: u32
        w.write_all(&(self.spells.len() as u32).to_le_bytes()).await?;

        // spells: TrainerSpell[amount_of_spells]
        for i in self.spells.iter() {
            i.tokio_write(w).await?;
        }

        // greeting: CString
        w.write_all(self.greeting.as_bytes()).await?;
        // Null terminator
        w.write_all(&[0]).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
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
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.astd_write(w).await?;

        // trainer_type: u32
        w.write_all(&self.trainer_type.to_le_bytes()).await?;

        // amount_of_spells: u32
        w.write_all(&(self.spells.len() as u32).to_le_bytes()).await?;

        // spells: TrainerSpell[amount_of_spells]
        for i in self.spells.iter() {
            i.astd_write(w).await?;
        }

        // greeting: CString
        w.write_all(self.greeting.as_bytes()).await?;
        // Null terminator
        w.write_all(&[0]).await?;

        Ok(())
    }
}

impl VariableSized for SMSG_TRAINER_LIST {
    fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // trainer_type: u32
        + 4 // amount_of_spells: u32
        + self.spells.iter().fold(0, |acc, x| acc + TrainerSpell::size()) // spells: TrainerSpell[amount_of_spells]
        + self.greeting.len() + 1 // greeting: CString and Null Terminator
    }
}

impl MaximumPossibleSized for SMSG_TRAINER_LIST {
    fn maximum_possible_size() -> usize {
        8 // guid: Guid
        + 4 // trainer_type: u32
        + 4 // amount_of_spells: u32
        + 4294967295 * TrainerSpell::maximum_possible_size() // spells: TrainerSpell[amount_of_spells]
        + 256 // greeting: CString
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

