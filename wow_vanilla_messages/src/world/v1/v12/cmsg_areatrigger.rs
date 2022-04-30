use std::convert::{TryFrom, TryInto};
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
#[derive(Copy)]
pub struct CMSG_AREATRIGGER {
    pub trigger_id: u32,
}

impl ClientMessageWrite for CMSG_AREATRIGGER {}

impl MessageBody for CMSG_AREATRIGGER {
    const OPCODE: u16 = 0x00b4;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // trigger_id: u32
        let trigger_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            trigger_id,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // trigger_id: u32
        w.write_all(&self.trigger_id.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for CMSG_AREATRIGGER {}

impl MaximumPossibleSized for CMSG_AREATRIGGER {
    fn maximum_possible_size() -> usize {
        4 // trigger_id: u32
    }
}

