use std::convert::{TryFrom, TryInto};
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_LEAVE_BATTLEFIELD {
    pub unknown1: u8,
    pub battle_ground_type_id: u8,
    pub unknown2: u16,
}

impl ClientMessageWrite for CMSG_LEAVE_BATTLEFIELD {}

impl MessageBody for CMSG_LEAVE_BATTLEFIELD {
    const OPCODE: u16 = 0x02e1;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(r)?;

        // battle_ground_type_id: u8
        let battle_ground_type_id = crate::util::read_u8_le(r)?;

        // unknown2: u16
        let unknown2 = crate::util::read_u16_le(r)?;

        Ok(Self {
            unknown1,
            battle_ground_type_id,
            unknown2,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        // battle_ground_type_id: u8
        w.write_all(&self.battle_ground_type_id.to_le_bytes())?;

        // unknown2: u16
        w.write_all(&self.unknown2.to_le_bytes())?;

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
            // unknown1: u8
            let unknown1 = crate::util::tokio_read_u8_le(r).await?;

            // battle_ground_type_id: u8
            let battle_ground_type_id = crate::util::tokio_read_u8_le(r).await?;

            // unknown2: u16
            let unknown2 = crate::util::tokio_read_u16_le(r).await?;

            Ok(Self {
                unknown1,
                battle_ground_type_id,
                unknown2,
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
            // unknown1: u8
            w.write_all(&self.unknown1.to_le_bytes()).await?;

            // battle_ground_type_id: u8
            w.write_all(&self.battle_ground_type_id.to_le_bytes()).await?;

            // unknown2: u16
            w.write_all(&self.unknown2.to_le_bytes()).await?;

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
            // unknown1: u8
            let unknown1 = crate::util::astd_read_u8_le(r).await?;

            // battle_ground_type_id: u8
            let battle_ground_type_id = crate::util::astd_read_u8_le(r).await?;

            // unknown2: u16
            let unknown2 = crate::util::astd_read_u16_le(r).await?;

            Ok(Self {
                unknown1,
                battle_ground_type_id,
                unknown2,
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
            // unknown1: u8
            w.write_all(&self.unknown1.to_le_bytes()).await?;

            // battle_ground_type_id: u8
            w.write_all(&self.battle_ground_type_id.to_le_bytes()).await?;

            // unknown2: u16
            w.write_all(&self.unknown2.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl CMSG_LEAVE_BATTLEFIELD {
    pub(crate) fn size() -> usize {
        0
        + 1 // unknown1: u8
        + 1 // battle_ground_type_id: u8
        + 2 // unknown2: u16
    }
}

