use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::MovementInfo;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_MOVE_NOT_ACTIVE_MOVER {
    pub old_mover: Guid,
    pub movement_info: MovementInfo,
}

impl CMSG_MOVE_NOT_ACTIVE_MOVER {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // old_mover: Guid
        w.write_all(&self.old_mover.guid().to_le_bytes())?;

        // movement_info: MovementInfo
        w.write_all(&self.movement_info.as_bytes()?)?;

        Ok(w)
    }
}

impl ClientMessage for CMSG_MOVE_NOT_ACTIVE_MOVER {
    fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // old_mover: Guid
        w.write_all(&self.old_mover.guid().to_le_bytes())?;

        // movement_info: MovementInfo
        w.write_all(&self.movement_info.as_bytes()?)?;

        Ok(w)
    }
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

}

impl CMSG_MOVE_NOT_ACTIVE_MOVER {
    pub fn size(&self) -> usize {
        0
        + 8 // old_mover: Guid
        + self.movement_info.size() // movement_info: MovementInfo
    }
}

