use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{SpellCastTargets, SpellCastTargetsError};
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_CAST_SPELL {
    pub spell: u32,
    pub targets: SpellCastTargets,
}

impl ClientMessageWrite for CMSG_CAST_SPELL {}

impl MessageBody for CMSG_CAST_SPELL {
    const OPCODE: u16 = 0x012e;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = CMSG_CAST_SPELLError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        // targets: SpellCastTargets
        let targets = SpellCastTargets::read(r)?;

        Ok(Self {
            spell,
            targets,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // targets: SpellCastTargets
        self.targets.write(w)?;

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
            // spell: u32
            let spell = crate::util::tokio_read_u32_le(r).await?;

            // targets: SpellCastTargets
            let targets = SpellCastTargets::tokio_read(r).await?;

            Ok(Self {
                spell,
                targets,
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
            // spell: u32
            w.write_all(&self.spell.to_le_bytes()).await?;

            // targets: SpellCastTargets
            self.targets.tokio_write(w).await?;

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
            // spell: u32
            let spell = crate::util::astd_read_u32_le(r).await?;

            // targets: SpellCastTargets
            let targets = SpellCastTargets::astd_read(r).await?;

            Ok(Self {
                spell,
                targets,
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
            // spell: u32
            w.write_all(&self.spell.to_le_bytes()).await?;

            // targets: SpellCastTargets
            self.targets.astd_write(w).await?;

            Ok(())
        })
    }

}

impl CMSG_CAST_SPELL {
    pub fn size(&self) -> usize {
        0
        + 4 // spell: u32
        + self.targets.size() // targets: SpellCastTargets
    }
}

#[derive(Debug)]
pub enum CMSG_CAST_SPELLError {
    Io(std::io::Error),
    SpellCastTargets(SpellCastTargetsError),
}

impl std::error::Error for CMSG_CAST_SPELLError {}
impl std::fmt::Display for CMSG_CAST_SPELLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::SpellCastTargets(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_CAST_SPELLError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<SpellCastTargetsError> for CMSG_CAST_SPELLError {
    fn from(e: SpellCastTargetsError) -> Self {
        Self::SpellCastTargets(e)
    }
}

