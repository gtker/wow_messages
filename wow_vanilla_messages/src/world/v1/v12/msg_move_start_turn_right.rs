use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::MovementInfo;
use crate::{ClientMessageWrite, ServerMessageWrite, MessageBody};
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
pub struct MSG_MOVE_START_TURN_RIGHT {
    pub info: MovementInfo,
}

impl ClientMessageWrite for MSG_MOVE_START_TURN_RIGHT {
    const OPCODE: u32 = 0xbd;

    fn size_without_size_field(&self) -> u16 {
        self.size() as u16
    }

}

impl ServerMessageWrite for MSG_MOVE_START_TURN_RIGHT {
    const OPCODE: u16 = 0xbd;

    fn size_without_size_field(&self) -> u16 {
        self.size() as u16
    }

}

impl MessageBody for MSG_MOVE_START_TURN_RIGHT {
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

impl VariableSized for MSG_MOVE_START_TURN_RIGHT {
    fn size(&self) -> usize {
        self.info.size() // info: MovementInfo
    }
}

impl MaximumPossibleSized for MSG_MOVE_START_TURN_RIGHT {
    fn maximum_possible_size() -> usize {
        MovementInfo::maximum_possible_size() // info: MovementInfo
    }
}

