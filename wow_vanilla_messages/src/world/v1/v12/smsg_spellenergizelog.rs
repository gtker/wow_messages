use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::PowerType;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_SPELLENERGIZELOG {
    pub victim_guid: Guid,
    pub caster_guid: Guid,
    pub spell: u32,
    pub power: PowerType,
    pub damage: u32,
}

impl SMSG_SPELLENERGIZELOG {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // victim_guid: PackedGuid
        w.write_all(&self.victim_guid.packed_guid())?;

        // caster_guid: PackedGuid
        w.write_all(&self.caster_guid.packed_guid())?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // power: PowerType
        w.write_all(&(self.power.as_int() as u32).to_le_bytes())?;

        // damage: u32
        w.write_all(&self.damage.to_le_bytes())?;

        Ok(w)
    }
}

impl ServerMessage for SMSG_SPELLENERGIZELOG {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // victim_guid: PackedGuid
        w.write_all(&self.victim_guid.packed_guid())?;

        // caster_guid: PackedGuid
        w.write_all(&self.caster_guid.packed_guid())?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // power: PowerType
        w.write_all(&(self.power.as_int() as u32).to_le_bytes())?;

        // damage: u32
        w.write_all(&self.damage.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0151;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_SPELLENERGIZELOGError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // victim_guid: PackedGuid
        let victim_guid = Guid::read_packed(r)?;

        // caster_guid: PackedGuid
        let caster_guid = Guid::read_packed(r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        // power: PowerType
        let power: PowerType = crate::util::read_u32_le(r)?.try_into()?;

        // damage: u32
        let damage = crate::util::read_u32_le(r)?;

        Ok(Self {
            victim_guid,
            caster_guid,
            spell,
            power,
            damage,
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
            // victim_guid: PackedGuid
            let victim_guid = Guid::tokio_read_packed(r).await?;

            // caster_guid: PackedGuid
            let caster_guid = Guid::tokio_read_packed(r).await?;

            // spell: u32
            let spell = crate::util::tokio_read_u32_le(r).await?;

            // power: PowerType
            let power: PowerType = crate::util::tokio_read_u32_le(r).await?.try_into()?;

            // damage: u32
            let damage = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                victim_guid,
                caster_guid,
                spell,
                power,
                damage,
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
            // victim_guid: PackedGuid
            let victim_guid = Guid::astd_read_packed(r).await?;

            // caster_guid: PackedGuid
            let caster_guid = Guid::astd_read_packed(r).await?;

            // spell: u32
            let spell = crate::util::astd_read_u32_le(r).await?;

            // power: PowerType
            let power: PowerType = crate::util::astd_read_u32_le(r).await?.try_into()?;

            // damage: u32
            let damage = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                victim_guid,
                caster_guid,
                spell,
                power,
                damage,
            })
        })
    }

}

impl SMSG_SPELLENERGIZELOG {
    pub fn size(&self) -> usize {
        0
        + self.victim_guid.size() // victim_guid: Guid
        + self.caster_guid.size() // caster_guid: Guid
        + 4 // spell: u32
        + 4 // power: PowerType
        + 4 // damage: u32
    }
}

#[derive(Debug)]
pub enum SMSG_SPELLENERGIZELOGError {
    Io(std::io::Error),
    Enum(crate::errors::EnumError),
}

impl std::error::Error for SMSG_SPELLENERGIZELOGError {}
impl std::fmt::Display for SMSG_SPELLENERGIZELOGError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Enum(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_SPELLENERGIZELOGError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<crate::errors::EnumError> for SMSG_SPELLENERGIZELOGError {
    fn from(e: crate::errors::EnumError) -> Self {
        Self::Enum(e)
    }
}

