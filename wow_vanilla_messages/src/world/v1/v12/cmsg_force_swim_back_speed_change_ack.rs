use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::MovementInfo;
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK {
    pub guid: Guid,
    pub counter: u32,
    pub movement_info: MovementInfo,
    pub new_speed: f32,
}

impl ClientMessageWrite for CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK {}

impl CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(97);
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // counter: u32
        w.write_all(&self.counter.to_le_bytes())?;

        // movement_info: MovementInfo
        w.write_all(&self.movement_info.as_bytes()?)?;

        // new_speed: f32
        w.write_all(&self.new_speed.to_le_bytes())?;

        Ok(w)
    }
}

impl MessageBody for CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK {
    const OPCODE: u16 = 0x02dd;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // counter: u32
        let counter = crate::util::read_u32_le(r)?;

        // movement_info: MovementInfo
        let movement_info = MovementInfo::read(r)?;

        // new_speed: f32
        let new_speed = crate::util::read_f32_le(r)?;
        Ok(Self {
            guid,
            counter,
            movement_info,
            new_speed,
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
            // guid: Guid
            let guid = Guid::tokio_read(r).await?;

            // counter: u32
            let counter = crate::util::tokio_read_u32_le(r).await?;

            // movement_info: MovementInfo
            let movement_info = MovementInfo::tokio_read(r).await?;

            // new_speed: f32
            let new_speed = crate::util::tokio_read_f32_le(r).await?;
            Ok(Self {
                guid,
                counter,
                movement_info,
                new_speed,
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
            // guid: Guid
            let guid = Guid::astd_read(r).await?;

            // counter: u32
            let counter = crate::util::astd_read_u32_le(r).await?;

            // movement_info: MovementInfo
            let movement_info = MovementInfo::astd_read(r).await?;

            // new_speed: f32
            let new_speed = crate::util::astd_read_f32_le(r).await?;
            Ok(Self {
                guid,
                counter,
                movement_info,
                new_speed,
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

impl CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK {
    pub fn size(&self) -> usize {
        0
        + 8 // guid: Guid
        + 4 // counter: u32
        + self.movement_info.size() // movement_info: MovementInfo
        + 4 // new_speed: f32
    }
}

