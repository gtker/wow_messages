use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::MovementInfo;
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_FORCE_RUN_SPEED_CHANGE_ACK {
    pub guid: Guid,
    pub counter: u32,
    pub movement_info: MovementInfo,
    pub new_speed: f32,
}

impl ClientMessageWrite for CMSG_FORCE_RUN_SPEED_CHANGE_ACK {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for CMSG_FORCE_RUN_SPEED_CHANGE_ACK {
    const OPCODE: u16 = 0x00e3;

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
        // guid: Guid
        self.guid.write(w)?;

        // counter: u32
        w.write_all(&self.counter.to_le_bytes())?;

        // movement_info: MovementInfo
        self.movement_info.write(w)?;

        // new_speed: f32
        w.write_all(&self.new_speed.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
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
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.tokio_write(w).await?;

        // counter: u32
        w.write_all(&self.counter.to_le_bytes()).await?;

        // movement_info: MovementInfo
        self.movement_info.tokio_write(w).await?;

        // new_speed: f32
        w.write_all(&self.new_speed.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
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
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.astd_write(w).await?;

        // counter: u32
        w.write_all(&self.counter.to_le_bytes()).await?;

        // movement_info: MovementInfo
        self.movement_info.astd_write(w).await?;

        // new_speed: f32
        w.write_all(&self.new_speed.to_le_bytes()).await?;

        Ok(())
    }

}

impl VariableSized for CMSG_FORCE_RUN_SPEED_CHANGE_ACK {
    fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // counter: u32
        + self.movement_info.size() // movement_info: MovementInfo
        + 4 // new_speed: f32
    }
}

impl MaximumPossibleSized for CMSG_FORCE_RUN_SPEED_CHANGE_ACK {
    fn maximum_possible_size() -> usize {
        8 // guid: Guid
        + 4 // counter: u32
        + MovementInfo::maximum_possible_size() // movement_info: MovementInfo
        + 4 // new_speed: f32
    }
}

