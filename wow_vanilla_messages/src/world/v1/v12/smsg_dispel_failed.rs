use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ServerMessage, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_DISPEL_FAILED {
    pub caster_guid: Guid,
    pub target_guid: Guid,
    pub spells: Vec<u32>,
}

impl ServerMessage for SMSG_DISPEL_FAILED {}

impl SMSG_DISPEL_FAILED {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // caster_guid: Guid
        w.write_all(&self.caster_guid.guid().to_le_bytes())?;

        // target_guid: Guid
        w.write_all(&self.target_guid.guid().to_le_bytes())?;

        // spells: u32[-]
        for i in self.spells.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(w)
    }
}

impl MessageBody for SMSG_DISPEL_FAILED {
    const OPCODE: u16 = 0x0262;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // caster_guid: Guid
        let caster_guid = Guid::read(r)?;

        // target_guid: Guid
        let target_guid = Guid::read(r)?;

        // spells: u32[-]
        let mut current_size = {
            8 // caster_guid: Guid
            + 8 // target_guid: Guid
        };
        let mut spells = Vec::with_capacity(body_size as usize - current_size);
        while current_size < (body_size as usize) {
            spells.push(crate::util::read_u32_le(r)?);
            current_size += 1;
        }

        Ok(Self {
            caster_guid,
            target_guid,
            spells,
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
            // caster_guid: Guid
            let caster_guid = Guid::tokio_read(r).await?;

            // target_guid: Guid
            let target_guid = Guid::tokio_read(r).await?;

            // spells: u32[-]
            let mut current_size = {
                8 // caster_guid: Guid
                + 8 // target_guid: Guid
            };
            let mut spells = Vec::with_capacity(body_size as usize - current_size);
            while current_size < (body_size as usize) {
                spells.push(crate::util::tokio_read_u32_le(r).await?);
                current_size += 1;
            }

            Ok(Self {
                caster_guid,
                target_guid,
                spells,
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
            // caster_guid: Guid
            let caster_guid = Guid::astd_read(r).await?;

            // target_guid: Guid
            let target_guid = Guid::astd_read(r).await?;

            // spells: u32[-]
            let mut current_size = {
                8 // caster_guid: Guid
                + 8 // target_guid: Guid
            };
            let mut spells = Vec::with_capacity(body_size as usize - current_size);
            while current_size < (body_size as usize) {
                spells.push(crate::util::astd_read_u32_le(r).await?);
                current_size += 1;
            }

            Ok(Self {
                caster_guid,
                target_guid,
                spells,
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

impl SMSG_DISPEL_FAILED {
    pub fn size(&self) -> usize {
        0
        + 8 // caster_guid: Guid
        + 8 // target_guid: Guid
        + self.spells.len() * core::mem::size_of::<u32>() // spells: u32[-]
    }
}

