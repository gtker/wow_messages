use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::SpellSchool;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_SPELLDAMAGESHIELD {
    pub victim_guid: Guid,
    pub caster_guid: Guid,
    pub damage: u32,
    pub school: SpellSchool,
}

impl SMSG_SPELLDAMAGESHIELD {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 21], std::io::Error> {
        let mut array_w = [0u8; 21];
        let mut w = array_w.as_mut_slice();
        // victim_guid: Guid
        w.write_all(&self.victim_guid.guid().to_le_bytes())?;

        // caster_guid: Guid
        w.write_all(&self.caster_guid.guid().to_le_bytes())?;

        // damage: u32
        w.write_all(&self.damage.to_le_bytes())?;

        // school: SpellSchool
        w.write_all(&(self.school.as_int() as u32).to_le_bytes())?;

        Ok(array_w)
    }
}

impl ServerMessage for SMSG_SPELLDAMAGESHIELD {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // victim_guid: Guid
        w.write_all(&self.victim_guid.guid().to_le_bytes())?;

        // caster_guid: Guid
        w.write_all(&self.caster_guid.guid().to_le_bytes())?;

        // damage: u32
        w.write_all(&self.damage.to_le_bytes())?;

        // school: SpellSchool
        w.write_all(&(self.school.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x024f;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        21
    }

    type Error = crate::errors::ParseError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // victim_guid: Guid
        let victim_guid = Guid::read(r)?;

        // caster_guid: Guid
        let caster_guid = Guid::read(r)?;

        // damage: u32
        let damage = crate::util::read_u32_le(r)?;

        // school: SpellSchool
        let school: SpellSchool = (crate::util::read_u32_le(r)? as u8).try_into()?;

        Ok(Self {
            victim_guid,
            caster_guid,
            damage,
            school,
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
            // victim_guid: Guid
            let victim_guid = Guid::tokio_read(r).await?;

            // caster_guid: Guid
            let caster_guid = Guid::tokio_read(r).await?;

            // damage: u32
            let damage = crate::util::tokio_read_u32_le(r).await?;

            // school: SpellSchool
            let school: SpellSchool = (crate::util::tokio_read_u32_le(r).await? as u8).try_into()?;

            Ok(Self {
                victim_guid,
                caster_guid,
                damage,
                school,
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
            // victim_guid: Guid
            let victim_guid = Guid::astd_read(r).await?;

            // caster_guid: Guid
            let caster_guid = Guid::astd_read(r).await?;

            // damage: u32
            let damage = crate::util::astd_read_u32_le(r).await?;

            // school: SpellSchool
            let school: SpellSchool = (crate::util::astd_read_u32_le(r).await? as u8).try_into()?;

            Ok(Self {
                victim_guid,
                caster_guid,
                damage,
                school,
            })
        })
    }

}

