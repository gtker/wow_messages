use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_LOOT_MASTER_GIVE {
    pub loot_guid: Guid,
    pub slot_id: u8,
    pub target_player_guid: Guid,
}

impl ClientMessageWrite for CMSG_LOOT_MASTER_GIVE {}

impl MessageBody for CMSG_LOOT_MASTER_GIVE {
    const OPCODE: u16 = 0x02a3;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // loot_guid: Guid
        let loot_guid = Guid::read(r)?;

        // slot_id: u8
        let slot_id = crate::util::read_u8_le(r)?;

        // target_player_guid: Guid
        let target_player_guid = Guid::read(r)?;

        Ok(Self {
            loot_guid,
            slot_id,
            target_player_guid,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // loot_guid: Guid
        self.loot_guid.write(w)?;

        // slot_id: u8
        w.write_all(&self.slot_id.to_le_bytes())?;

        // target_player_guid: Guid
        self.target_player_guid.write(w)?;

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
            // loot_guid: Guid
            let loot_guid = Guid::tokio_read(r).await?;

            // slot_id: u8
            let slot_id = crate::util::tokio_read_u8_le(r).await?;

            // target_player_guid: Guid
            let target_player_guid = Guid::tokio_read(r).await?;

            Ok(Self {
                loot_guid,
                slot_id,
                target_player_guid,
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
            // loot_guid: Guid
            self.loot_guid.tokio_write(w).await?;

            // slot_id: u8
            w.write_all(&self.slot_id.to_le_bytes()).await?;

            // target_player_guid: Guid
            self.target_player_guid.tokio_write(w).await?;

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
            // loot_guid: Guid
            let loot_guid = Guid::astd_read(r).await?;

            // slot_id: u8
            let slot_id = crate::util::astd_read_u8_le(r).await?;

            // target_player_guid: Guid
            let target_player_guid = Guid::astd_read(r).await?;

            Ok(Self {
                loot_guid,
                slot_id,
                target_player_guid,
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
            // loot_guid: Guid
            self.loot_guid.astd_write(w).await?;

            // slot_id: u8
            w.write_all(&self.slot_id.to_le_bytes()).await?;

            // target_player_guid: Guid
            self.target_player_guid.astd_write(w).await?;

            Ok(())
        })
    }

}

impl ConstantSized for CMSG_LOOT_MASTER_GIVE {}

impl MaximumPossibleSized for CMSG_LOOT_MASTER_GIVE {
    fn maximum_possible_size() -> usize {
        0
        + 8 // loot_guid: Guid
        + 1 // slot_id: u8
        + 8 // target_player_guid: Guid
    }
}

