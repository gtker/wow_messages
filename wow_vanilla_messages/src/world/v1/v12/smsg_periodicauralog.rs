use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{AuraLog, AuraLogError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_PERIODICAURALOG {
    pub target: Guid,
    pub caster: Guid,
    pub spell: u32,
    pub auras: Vec<AuraLog>,
}

impl ServerMessageWrite for SMSG_PERIODICAURALOG {}

impl SMSG_PERIODICAURALOG {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(73014444058);
        // target: PackedGuid
        w.write_all(&self.target.packed_guid())?;

        // caster: PackedGuid
        w.write_all(&self.caster.packed_guid())?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // amount_of_auras: u32
        w.write_all(&(self.auras.len() as u32).to_le_bytes())?;

        // auras: AuraLog[amount_of_auras]
        for i in self.auras.iter() {
            w.write_all(&(i.as_bytes()?))?;
        }

        Ok(w)
    }
}

impl MessageBody for SMSG_PERIODICAURALOG {
    const OPCODE: u16 = 0x024e;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_PERIODICAURALOGError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // target: PackedGuid
        let target = Guid::read_packed(r)?;

        // caster: PackedGuid
        let caster = Guid::read_packed(r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        // amount_of_auras: u32
        let amount_of_auras = crate::util::read_u32_le(r)?;

        // auras: AuraLog[amount_of_auras]
        let mut auras = Vec::with_capacity(amount_of_auras as usize);
        for i in 0..amount_of_auras {
            auras.push(AuraLog::read(r)?);
        }

        Ok(Self {
            target,
            caster,
            spell,
            auras,
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
            // target: PackedGuid
            let target = Guid::tokio_read_packed(r).await?;

            // caster: PackedGuid
            let caster = Guid::tokio_read_packed(r).await?;

            // spell: u32
            let spell = crate::util::tokio_read_u32_le(r).await?;

            // amount_of_auras: u32
            let amount_of_auras = crate::util::tokio_read_u32_le(r).await?;

            // auras: AuraLog[amount_of_auras]
            let mut auras = Vec::with_capacity(amount_of_auras as usize);
            for i in 0..amount_of_auras {
                auras.push(AuraLog::tokio_read(r).await?);
            }

            Ok(Self {
                target,
                caster,
                spell,
                auras,
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
            // target: PackedGuid
            let target = Guid::astd_read_packed(r).await?;

            // caster: PackedGuid
            let caster = Guid::astd_read_packed(r).await?;

            // spell: u32
            let spell = crate::util::astd_read_u32_le(r).await?;

            // amount_of_auras: u32
            let amount_of_auras = crate::util::astd_read_u32_le(r).await?;

            // auras: AuraLog[amount_of_auras]
            let mut auras = Vec::with_capacity(amount_of_auras as usize);
            for i in 0..amount_of_auras {
                auras.push(AuraLog::astd_read(r).await?);
            }

            Ok(Self {
                target,
                caster,
                spell,
                auras,
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

impl SMSG_PERIODICAURALOG {
    pub fn size(&self) -> usize {
        0
        + self.target.size() // target: Guid
        + self.caster.size() // caster: Guid
        + 4 // spell: u32
        + 4 // amount_of_auras: u32
        + self.auras.iter().fold(0, |acc, x| acc + x.size()) // auras: AuraLog[amount_of_auras]
    }
}

#[derive(Debug)]
pub enum SMSG_PERIODICAURALOGError {
    Io(std::io::Error),
    AuraLog(AuraLogError),
}

impl std::error::Error for SMSG_PERIODICAURALOGError {}
impl std::fmt::Display for SMSG_PERIODICAURALOGError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::AuraLog(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_PERIODICAURALOGError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<AuraLogError> for SMSG_PERIODICAURALOGError {
    fn from(e: AuraLogError) -> Self {
        Self::AuraLog(e)
    }
}

