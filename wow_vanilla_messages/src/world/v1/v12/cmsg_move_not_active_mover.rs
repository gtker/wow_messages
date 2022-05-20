use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::MovementInfo;
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_MOVE_NOT_ACTIVE_MOVER {
    pub old_mover: Guid,
    pub movement_info: MovementInfo,
}

impl ClientMessageWrite for CMSG_MOVE_NOT_ACTIVE_MOVER {}

impl MessageBody for CMSG_MOVE_NOT_ACTIVE_MOVER {
    const OPCODE: u16 = 0x02d1;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // old_mover: Guid
        let old_mover = Guid::read(r)?;

        // movement_info: MovementInfo
        let movement_info = MovementInfo::read(r)?;

        Ok(Self {
            old_mover,
            movement_info,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // old_mover: Guid
        self.old_mover.write(w)?;

        // movement_info: MovementInfo
        self.movement_info.write(w)?;

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
            // old_mover: Guid
            let old_mover = Guid::tokio_read(r).await?;

            // movement_info: MovementInfo
            let movement_info = MovementInfo::tokio_read(r).await?;

            Ok(Self {
                old_mover,
                movement_info,
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
            // old_mover: Guid
            self.old_mover.tokio_write(w).await?;

            // movement_info: MovementInfo
            self.movement_info.tokio_write(w).await?;

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
            // old_mover: Guid
            let old_mover = Guid::astd_read(r).await?;

            // movement_info: MovementInfo
            let movement_info = MovementInfo::astd_read(r).await?;

            Ok(Self {
                old_mover,
                movement_info,
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
            // old_mover: Guid
            self.old_mover.astd_write(w).await?;

            // movement_info: MovementInfo
            self.movement_info.astd_write(w).await?;

            Ok(())
        })
    }

}

impl CMSG_MOVE_NOT_ACTIVE_MOVER {
    pub fn size(&self) -> usize {
        0
        + 8 // old_mover: Guid
        + self.movement_info.size() // movement_info: MovementInfo
    }
}

