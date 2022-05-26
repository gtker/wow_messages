use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_LOOT_MASTER_GIVE {
    pub loot_guid: Guid,
    pub slot_id: u8,
    pub target_player_guid: Guid,
}

impl CMSG_LOOT_MASTER_GIVE {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 17], std::io::Error> {
        let mut array_w = [0u8; 17];
        let mut w = array_w.as_mut_slice();
        // loot_guid: Guid
        w.write_all(&self.loot_guid.guid().to_le_bytes())?;

        // slot_id: u8
        w.write_all(&self.slot_id.to_le_bytes())?;

        // target_player_guid: Guid
        w.write_all(&self.target_player_guid.guid().to_le_bytes())?;

        Ok(array_w)
    }
}

impl ClientMessage for CMSG_LOOT_MASTER_GIVE {
    fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(17);
        // loot_guid: Guid
        w.write_all(&self.loot_guid.guid().to_le_bytes())?;

        // slot_id: u8
        w.write_all(&self.slot_id.to_le_bytes())?;

        // target_player_guid: Guid
        w.write_all(&self.target_player_guid.guid().to_le_bytes())?;

        Ok(w)
    }
    const OPCODE: u16 = 0x02a3;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        17
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

}

