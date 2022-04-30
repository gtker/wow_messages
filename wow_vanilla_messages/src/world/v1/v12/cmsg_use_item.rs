use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{SpellCastTargets, SpellCastTargetsError};
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_USE_ITEM {
    pub bag_index: u8,
    pub bag_slot: u8,
    pub spell_index: u8,
    pub targets: SpellCastTargets,
}

impl ClientMessageWrite for CMSG_USE_ITEM {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for CMSG_USE_ITEM {
    const OPCODE: u16 = 0x00ab;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = CMSG_USE_ITEMError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // bag_index: u8
        let bag_index = crate::util::read_u8_le(r)?;

        // bag_slot: u8
        let bag_slot = crate::util::read_u8_le(r)?;

        // spell_index: u8
        let spell_index = crate::util::read_u8_le(r)?;

        // targets: SpellCastTargets
        let targets = SpellCastTargets::read(r)?;

        Ok(Self {
            bag_index,
            bag_slot,
            spell_index,
            targets,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // bag_index: u8
        w.write_all(&self.bag_index.to_le_bytes())?;

        // bag_slot: u8
        w.write_all(&self.bag_slot.to_le_bytes())?;

        // spell_index: u8
        w.write_all(&self.spell_index.to_le_bytes())?;

        // targets: SpellCastTargets
        self.targets.write(w)?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // bag_index: u8
        let bag_index = crate::util::tokio_read_u8_le(r).await?;

        // bag_slot: u8
        let bag_slot = crate::util::tokio_read_u8_le(r).await?;

        // spell_index: u8
        let spell_index = crate::util::tokio_read_u8_le(r).await?;

        // targets: SpellCastTargets
        let targets = SpellCastTargets::tokio_read(r).await?;

        Ok(Self {
            bag_index,
            bag_slot,
            spell_index,
            targets,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // bag_index: u8
        w.write_all(&self.bag_index.to_le_bytes()).await?;

        // bag_slot: u8
        w.write_all(&self.bag_slot.to_le_bytes()).await?;

        // spell_index: u8
        w.write_all(&self.spell_index.to_le_bytes()).await?;

        // targets: SpellCastTargets
        self.targets.tokio_write(w).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // bag_index: u8
        let bag_index = crate::util::astd_read_u8_le(r).await?;

        // bag_slot: u8
        let bag_slot = crate::util::astd_read_u8_le(r).await?;

        // spell_index: u8
        let spell_index = crate::util::astd_read_u8_le(r).await?;

        // targets: SpellCastTargets
        let targets = SpellCastTargets::astd_read(r).await?;

        Ok(Self {
            bag_index,
            bag_slot,
            spell_index,
            targets,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // bag_index: u8
        w.write_all(&self.bag_index.to_le_bytes()).await?;

        // bag_slot: u8
        w.write_all(&self.bag_slot.to_le_bytes()).await?;

        // spell_index: u8
        w.write_all(&self.spell_index.to_le_bytes()).await?;

        // targets: SpellCastTargets
        self.targets.astd_write(w).await?;

        Ok(())
    }
}

impl VariableSized for CMSG_USE_ITEM {
    fn size(&self) -> usize {
        1 // bag_index: u8
        + 1 // bag_slot: u8
        + 1 // spell_index: u8
        + self.targets.size() // targets: SpellCastTargets
    }
}

impl MaximumPossibleSized for CMSG_USE_ITEM {
    fn maximum_possible_size() -> usize {
        1 // bag_index: u8
        + 1 // bag_slot: u8
        + 1 // spell_index: u8
        + SpellCastTargets::maximum_possible_size() // targets: SpellCastTargets
    }
}

#[derive(Debug)]
pub enum CMSG_USE_ITEMError {
    Io(std::io::Error),
    SpellCastTargets(SpellCastTargetsError),
}

impl std::error::Error for CMSG_USE_ITEMError {}
impl std::fmt::Display for CMSG_USE_ITEMError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::SpellCastTargets(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_USE_ITEMError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<SpellCastTargetsError> for CMSG_USE_ITEMError {
    fn from(e: SpellCastTargetsError) -> Self {
        Self::SpellCastTargets(e)
    }
}

