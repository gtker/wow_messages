use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::MovementInfo;
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use crate::AsyncReadWrite;
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

impl ClientMessageWrite for CMSG_MOVE_NOT_ACTIVE_MOVER {
    const OPCODE: u32 = 0x2d1;

    fn size_without_size_field(&self) -> u16 {
        self.size() as u16
    }

}

impl MessageBody for CMSG_MOVE_NOT_ACTIVE_MOVER {
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

