use std::convert::{TryFrom, TryInto};
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
pub struct SMSG_DUEL_COMPLETE {
    pub ended_without_interruption: u8,
}

impl ServerMessageWrite for SMSG_DUEL_COMPLETE {
    const OPCODE: u16 = 0x16a;

    fn size_without_size_field(&self) -> u16 {
        Self::size() as u16
    }

}

impl MessageBody for SMSG_DUEL_COMPLETE {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // ended_without_interruption: u8
        let ended_without_interruption = crate::util::read_u8_le(r)?;

        Ok(Self {
            ended_without_interruption,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // ended_without_interruption: u8
        w.write_all(&self.ended_without_interruption.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_DUEL_COMPLETE {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_DUEL_COMPLETE {
    fn maximum_possible_size() -> usize {
        1 // ended_without_interruption: u8
    }
}

