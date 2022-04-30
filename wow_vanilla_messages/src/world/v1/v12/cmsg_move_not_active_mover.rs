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
pub struct CMSG_MOVE_NOT_ACTIVE_MOVER {
    pub old_mover: Guid,
    pub movement_info: MovementInfo,
}

impl ClientMessageWrite for CMSG_MOVE_NOT_ACTIVE_MOVER {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for CMSG_MOVE_NOT_ACTIVE_MOVER {
    const OPCODE: u16 = 0x02d1;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

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

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // old_mover: Guid
        self.old_mover.write(w)?;

        // movement_info: MovementInfo
        self.movement_info.write(w)?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // old_mover: Guid
        let old_mover = Guid::tokio_read(r).await?;

        // movement_info: MovementInfo
        let movement_info = MovementInfo::tokio_read(r).await?;

        Ok(Self {
            old_mover,
            movement_info,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // old_mover: Guid
        self.old_mover.tokio_write(w).await?;

        // movement_info: MovementInfo
        self.movement_info.tokio_write(w).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // old_mover: Guid
        let old_mover = Guid::astd_read(r).await?;

        // movement_info: MovementInfo
        let movement_info = MovementInfo::astd_read(r).await?;

        Ok(Self {
            old_mover,
            movement_info,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // old_mover: Guid
        self.old_mover.astd_write(w).await?;

        // movement_info: MovementInfo
        self.movement_info.astd_write(w).await?;

        Ok(())
    }
}

impl VariableSized for CMSG_MOVE_NOT_ACTIVE_MOVER {
    fn size(&self) -> usize {
        8 // old_mover: Guid
        + self.movement_info.size() // movement_info: MovementInfo
    }
}

impl MaximumPossibleSized for CMSG_MOVE_NOT_ACTIVE_MOVER {
    fn maximum_possible_size() -> usize {
        8 // old_mover: Guid
        + MovementInfo::maximum_possible_size() // movement_info: MovementInfo
    }
}

