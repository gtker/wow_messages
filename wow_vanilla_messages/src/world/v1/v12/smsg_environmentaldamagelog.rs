use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{EnvironmentalDamageType, EnvironmentalDamageTypeError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_ENVIRONMENTALDAMAGELOG {
    pub guid: Guid,
    pub damage_type: EnvironmentalDamageType,
    pub damage: u32,
    pub absorb: u32,
    pub resist: u32,
}

impl ServerMessageWrite for SMSG_ENVIRONMENTALDAMAGELOG {}

impl MessageBody for SMSG_ENVIRONMENTALDAMAGELOG {
    const OPCODE: u16 = 0x01fc;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_ENVIRONMENTALDAMAGELOGError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // damage_type: EnvironmentalDamageType
        let damage_type = EnvironmentalDamageType::read(r)?;

        // damage: u32
        let damage = crate::util::read_u32_le(r)?;

        // absorb: u32
        let absorb = crate::util::read_u32_le(r)?;

        // resist: u32
        let resist = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            damage_type,
            damage,
            absorb,
            resist,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // damage_type: EnvironmentalDamageType
        self.damage_type.write(w)?;

        // damage: u32
        w.write_all(&self.damage.to_le_bytes())?;

        // absorb: u32
        w.write_all(&self.absorb.to_le_bytes())?;

        // resist: u32
        w.write_all(&self.resist.to_le_bytes())?;

        Ok(())
    }

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

            // damage_type: EnvironmentalDamageType
            let damage_type = EnvironmentalDamageType::tokio_read(r).await?;

            // damage: u32
            let damage = crate::util::tokio_read_u32_le(r).await?;

            // absorb: u32
            let absorb = crate::util::tokio_read_u32_le(r).await?;

            // resist: u32
            let resist = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                guid,
                damage_type,
                damage,
                absorb,
                resist,
            })
        })
    }

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
            // guid: Guid
            self.guid.tokio_write(w).await?;

            // damage_type: EnvironmentalDamageType
            self.damage_type.tokio_write(w).await?;

            // damage: u32
            w.write_all(&self.damage.to_le_bytes()).await?;

            // absorb: u32
            w.write_all(&self.absorb.to_le_bytes()).await?;

            // resist: u32
            w.write_all(&self.resist.to_le_bytes()).await?;

            Ok(())
        })
    }

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

            // damage_type: EnvironmentalDamageType
            let damage_type = EnvironmentalDamageType::astd_read(r).await?;

            // damage: u32
            let damage = crate::util::astd_read_u32_le(r).await?;

            // absorb: u32
            let absorb = crate::util::astd_read_u32_le(r).await?;

            // resist: u32
            let resist = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                guid,
                damage_type,
                damage,
                absorb,
                resist,
            })
        })
    }

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
            // guid: Guid
            self.guid.astd_write(w).await?;

            // damage_type: EnvironmentalDamageType
            self.damage_type.astd_write(w).await?;

            // damage: u32
            w.write_all(&self.damage.to_le_bytes()).await?;

            // absorb: u32
            w.write_all(&self.absorb.to_le_bytes()).await?;

            // resist: u32
            w.write_all(&self.resist.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl ConstantSized for SMSG_ENVIRONMENTALDAMAGELOG {}

impl MaximumPossibleSized for SMSG_ENVIRONMENTALDAMAGELOG {
    fn maximum_possible_size() -> usize {
        0
        + 8 // guid: Guid
        + 4 // damage_type: EnvironmentalDamageType
        + 4 // damage: u32
        + 4 // absorb: u32
        + 4 // resist: u32
    }
}

#[derive(Debug)]
pub enum SMSG_ENVIRONMENTALDAMAGELOGError {
    Io(std::io::Error),
    EnvironmentalDamageType(EnvironmentalDamageTypeError),
}

impl std::error::Error for SMSG_ENVIRONMENTALDAMAGELOGError {}
impl std::fmt::Display for SMSG_ENVIRONMENTALDAMAGELOGError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::EnvironmentalDamageType(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_ENVIRONMENTALDAMAGELOGError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<EnvironmentalDamageTypeError> for SMSG_ENVIRONMENTALDAMAGELOGError {
    fn from(e: EnvironmentalDamageTypeError) -> Self {
        Self::EnvironmentalDamageType(e)
    }
}

