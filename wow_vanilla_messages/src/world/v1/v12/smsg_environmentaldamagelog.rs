use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::EnvironmentalDamageType;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_ENVIRONMENTALDAMAGELOG {
    pub guid: Guid,
    pub damage_type: EnvironmentalDamageType,
    pub damage: u32,
    pub absorb: u32,
    pub resist: u32,
}

impl SMSG_ENVIRONMENTALDAMAGELOG {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 24], std::io::Error> {
        let mut array_w = [0u8; 24];
        let mut w = array_w.as_mut_slice();
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // damage_type: EnvironmentalDamageType
        w.write_all(&(self.damage_type.as_int() as u32).to_le_bytes())?;

        // damage: u32
        w.write_all(&self.damage.to_le_bytes())?;

        // absorb: u32
        w.write_all(&self.absorb.to_le_bytes())?;

        // resist: u32
        w.write_all(&self.resist.to_le_bytes())?;

        Ok(array_w)
    }
}

impl ServerMessage for SMSG_ENVIRONMENTALDAMAGELOG {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // damage_type: EnvironmentalDamageType
        w.write_all(&(self.damage_type.as_int() as u32).to_le_bytes())?;

        // damage: u32
        w.write_all(&self.damage.to_le_bytes())?;

        // absorb: u32
        w.write_all(&self.absorb.to_le_bytes())?;

        // resist: u32
        w.write_all(&self.resist.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x01fc;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        24
    }

    type Error = SMSG_ENVIRONMENTALDAMAGELOGError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // damage_type: EnvironmentalDamageType
        let damage_type: EnvironmentalDamageType = crate::util::read_u32_le(r)?.try_into()?;

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

            // damage_type: EnvironmentalDamageType
            let damage_type: EnvironmentalDamageType = crate::util::tokio_read_u32_le(r).await?.try_into()?;

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

            // damage_type: EnvironmentalDamageType
            let damage_type: EnvironmentalDamageType = crate::util::astd_read_u32_le(r).await?.try_into()?;

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

}

#[derive(Debug)]
pub enum SMSG_ENVIRONMENTALDAMAGELOGError {
    Io(std::io::Error),
    Enum(crate::errors::EnumError),
}

impl std::error::Error for SMSG_ENVIRONMENTALDAMAGELOGError {}
impl std::fmt::Display for SMSG_ENVIRONMENTALDAMAGELOGError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Enum(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_ENVIRONMENTALDAMAGELOGError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<crate::errors::EnumError> for SMSG_ENVIRONMENTALDAMAGELOGError {
    fn from(e: crate::errors::EnumError) -> Self {
        Self::Enum(e)
    }
}

