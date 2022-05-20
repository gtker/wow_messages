use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_ATTACKERSTATEUPDATE {
    pub hit_info: u32,
    pub attacker: Guid,
    pub target: Guid,
    pub total_damage: u32,
}

impl ServerMessageWrite for SMSG_ATTACKERSTATEUPDATE {}

impl MessageBody for SMSG_ATTACKERSTATEUPDATE {
    const OPCODE: u16 = 0x014a;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // hit_info: u32
        let hit_info = crate::util::read_u32_le(r)?;

        // attacker: PackedGuid
        let attacker = Guid::read_packed(r)?;

        // target: PackedGuid
        let target = Guid::read_packed(r)?;

        // total_damage: u32
        let total_damage = crate::util::read_u32_le(r)?;

        Ok(Self {
            hit_info,
            attacker,
            target,
            total_damage,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // hit_info: u32
        w.write_all(&self.hit_info.to_le_bytes())?;

        // attacker: PackedGuid
        w.write_all(&self.attacker.packed_guid())?;

        // target: PackedGuid
        w.write_all(&self.target.packed_guid())?;

        // total_damage: u32
        w.write_all(&self.total_damage.to_le_bytes())?;

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
            // hit_info: u32
            let hit_info = crate::util::tokio_read_u32_le(r).await?;

            // attacker: PackedGuid
            let attacker = Guid::tokio_read_packed(r).await?;

            // target: PackedGuid
            let target = Guid::tokio_read_packed(r).await?;

            // total_damage: u32
            let total_damage = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                hit_info,
                attacker,
                target,
                total_damage,
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
            // hit_info: u32
            w.write_all(&self.hit_info.to_le_bytes()).await?;

            // attacker: PackedGuid
            w.write_all(&self.attacker.packed_guid()).await?;

            // target: PackedGuid
            w.write_all(&self.target.packed_guid()).await?;

            // total_damage: u32
            w.write_all(&self.total_damage.to_le_bytes()).await?;

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
            // hit_info: u32
            let hit_info = crate::util::astd_read_u32_le(r).await?;

            // attacker: PackedGuid
            let attacker = Guid::astd_read_packed(r).await?;

            // target: PackedGuid
            let target = Guid::astd_read_packed(r).await?;

            // total_damage: u32
            let total_damage = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                hit_info,
                attacker,
                target,
                total_damage,
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
            // hit_info: u32
            w.write_all(&self.hit_info.to_le_bytes()).await?;

            // attacker: PackedGuid
            w.write_all(&self.attacker.packed_guid()).await?;

            // target: PackedGuid
            w.write_all(&self.target.packed_guid()).await?;

            // total_damage: u32
            w.write_all(&self.total_damage.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl SMSG_ATTACKERSTATEUPDATE {
    pub fn size(&self) -> usize {
        0
        + 4 // hit_info: u32
        + self.attacker.size() // attacker: Guid
        + self.target.size() // target: Guid
        + 4 // total_damage: u32
    }
}

