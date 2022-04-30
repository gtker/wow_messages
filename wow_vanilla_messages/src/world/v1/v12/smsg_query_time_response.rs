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
pub struct SMSG_QUERY_TIME_RESPONSE {
    pub time: u32,
}

impl ServerMessageWrite for SMSG_QUERY_TIME_RESPONSE {}

impl MessageBody for SMSG_QUERY_TIME_RESPONSE {
    const OPCODE: u16 = 0x01cf;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // time: u32
        let time = crate::util::read_u32_le(r)?;

        Ok(Self {
            time,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // time: u32
        w.write_all(&self.time.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_QUERY_TIME_RESPONSE {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_QUERY_TIME_RESPONSE {
    fn maximum_possible_size() -> usize {
        4 // time: u32
    }
}

