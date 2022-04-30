use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{TimerType, TimerTypeError};
use crate::{ServerMessageWrite, MessageBody};
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
#[derive(Copy)]
pub struct SMSG_STOP_MIRROR_TIMER {
    pub timer: TimerType,
}

impl ServerMessageWrite for SMSG_STOP_MIRROR_TIMER {
    const OPCODE: u16 = 0x1db;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (Self::size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as ServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (Self::size() + 2) as u16, <Self as ServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl MessageBody for SMSG_STOP_MIRROR_TIMER {
    type Error = SMSG_STOP_MIRROR_TIMERError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // timer: TimerType
        let timer = TimerType::read(r)?;

        Ok(Self {
            timer,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // timer: TimerType
        self.timer.write(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_STOP_MIRROR_TIMER {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_STOP_MIRROR_TIMER {
    fn maximum_possible_size() -> usize {
        TimerType::size() // timer: TimerType
    }
}

#[derive(Debug)]
pub enum SMSG_STOP_MIRROR_TIMERError {
    Io(std::io::Error),
    TimerType(TimerTypeError),
}

impl std::error::Error for SMSG_STOP_MIRROR_TIMERError {}
impl std::fmt::Display for SMSG_STOP_MIRROR_TIMERError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::TimerType(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_STOP_MIRROR_TIMERError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<TimerTypeError> for SMSG_STOP_MIRROR_TIMERError {
    fn from(e: TimerTypeError) -> Self {
        Self::TimerType(e)
    }
}

