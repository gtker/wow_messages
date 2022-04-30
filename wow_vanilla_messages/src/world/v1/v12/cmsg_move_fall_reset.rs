use std::convert::{TryFrom, TryInto};
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
pub struct CMSG_MOVE_FALL_RESET {
    pub info: MovementInfo,
}

impl ClientMessageWrite for CMSG_MOVE_FALL_RESET {
    const OPCODE: u32 = 0x2ca;

    fn size_without_size_field(&self) -> u16 {
        self.size() as u16
    }

}

impl MessageBody for CMSG_MOVE_FALL_RESET {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // info: MovementInfo
        let info = MovementInfo::read(r)?;

        Ok(Self {
            info,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // info: MovementInfo
        self.info.write(w)?;

        Ok(())
    }
}

impl VariableSized for CMSG_MOVE_FALL_RESET {
    fn size(&self) -> usize {
        self.info.size() // info: MovementInfo
    }
}

impl MaximumPossibleSized for CMSG_MOVE_FALL_RESET {
    fn maximum_possible_size() -> usize {
        MovementInfo::maximum_possible_size() // info: MovementInfo
    }
}

