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
pub struct SMSG_DISPEL_FAILED {
    pub caster_guid: Guid,
    pub target_guid: Guid,
    pub spells: Vec<u32>,
}

impl ServerMessageWrite for SMSG_DISPEL_FAILED {}

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
        // caster_guid: Guid
        self.caster_guid.write(w)?;

        // target_guid: Guid
        self.target_guid.write(w)?;

        // spells: u32[-]
        for i in self.spells.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

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
            // caster_guid: Guid
            self.caster_guid.tokio_write(w).await?;

            // target_guid: Guid
            self.target_guid.tokio_write(w).await?;

            // spells: u32[-]
            for i in self.spells.iter() {
                w.write_all(&i.to_le_bytes()).await?;
            }

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
            // caster_guid: Guid
            self.caster_guid.astd_write(w).await?;

            // target_guid: Guid
            self.target_guid.astd_write(w).await?;

            // spells: u32[-]
            for i in self.spells.iter() {
                w.write_all(&i.to_le_bytes()).await?;
            }

            Ok(())
        })
    }

}

impl VariableSized for SMSG_DISPEL_FAILED {
    fn size(&self) -> usize {
        0
        + 8 // caster_guid: Guid
        + 8 // target_guid: Guid
        + self.spells.len() * core::mem::size_of::<u32>() // spells: u32[-]
    }
}

impl MaximumPossibleSized for SMSG_DISPEL_FAILED {
    fn maximum_possible_size() -> usize {
        65535 // Capped at u16::MAX due to size field.
    }
}

