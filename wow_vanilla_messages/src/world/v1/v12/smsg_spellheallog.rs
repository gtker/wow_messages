use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_SPELLHEALLOG {
    pub victim_guid: Guid,
    pub caster_guid: Guid,
    pub id: u32,
    pub damage: u32,
    pub critical: u8,
}

impl ServerMessageWrite for SMSG_SPELLHEALLOG {}

impl MessageBody for SMSG_SPELLHEALLOG {
    const OPCODE: u16 = 0x0150;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // victim_guid: PackedGuid
        let victim_guid = Guid::read_packed(r)?;

        // caster_guid: PackedGuid
        let caster_guid = Guid::read_packed(r)?;

        // id: u32
        let id = crate::util::read_u32_le(r)?;

        // damage: u32
        let damage = crate::util::read_u32_le(r)?;

        // critical: u8
        let critical = crate::util::read_u8_le(r)?;

        Ok(Self {
            victim_guid,
            caster_guid,
            id,
            damage,
            critical,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // victim_guid: PackedGuid
        self.victim_guid.write_packed(w)?;

        // caster_guid: PackedGuid
        self.caster_guid.write_packed(w)?;

        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // damage: u32
        w.write_all(&self.damage.to_le_bytes())?;

        // critical: u8
        w.write_all(&self.critical.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
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

            // id: u32
            let id = crate::util::tokio_read_u32_le(r).await?;

            // damage: u32
            let damage = crate::util::tokio_read_u32_le(r).await?;

            // critical: u8
            let critical = crate::util::tokio_read_u8_le(r).await?;

            Ok(Self {
                victim_guid,
                caster_guid,
                id,
                damage,
                critical,
            })
        })
    }

    #[cfg(feature = "async_tokio")]
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
            // victim_guid: PackedGuid
            self.victim_guid.tokio_write_packed(w).await?;

            // caster_guid: PackedGuid
            self.caster_guid.tokio_write_packed(w).await?;

            // id: u32
            w.write_all(&self.id.to_le_bytes()).await?;

            // damage: u32
            w.write_all(&self.damage.to_le_bytes()).await?;

            // critical: u8
            w.write_all(&self.critical.to_le_bytes()).await?;

            Ok(())
        })
    }

    #[cfg(feature = "async_std")]
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

            // id: u32
            let id = crate::util::astd_read_u32_le(r).await?;

            // damage: u32
            let damage = crate::util::astd_read_u32_le(r).await?;

            // critical: u8
            let critical = crate::util::astd_read_u8_le(r).await?;

            Ok(Self {
                victim_guid,
                caster_guid,
                id,
                damage,
                critical,
            })
        })
    }

    #[cfg(feature = "async_std")]
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
            // victim_guid: PackedGuid
            self.victim_guid.astd_write_packed(w).await?;

            // caster_guid: PackedGuid
            self.caster_guid.astd_write_packed(w).await?;

            // id: u32
            w.write_all(&self.id.to_le_bytes()).await?;

            // damage: u32
            w.write_all(&self.damage.to_le_bytes()).await?;

            // critical: u8
            w.write_all(&self.critical.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl VariableSized for SMSG_SPELLHEALLOG {
    fn size(&self) -> usize {
        0
        + self.victim_guid.size() // victim_guid: Guid
        + self.caster_guid.size() // caster_guid: Guid
        + 4 // id: u32
        + 4 // damage: u32
        + 1 // critical: u8
    }
}

impl MaximumPossibleSized for SMSG_SPELLHEALLOG {
    fn maximum_possible_size() -> usize {
        0
        + 9 // victim_guid: Guid
        + 9 // caster_guid: Guid
        + 4 // id: u32
        + 4 // damage: u32
        + 1 // critical: u8
    }
}

