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
pub struct CMSG_LEARN_TALENT {
    pub talent_id: u32,
    pub requested_rank: u32,
}

impl ClientMessageWrite for CMSG_LEARN_TALENT {}

impl MessageBody for CMSG_LEARN_TALENT {
    const OPCODE: u16 = 0x0251;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // talent_id: u32
        let talent_id = crate::util::read_u32_le(r)?;

        // requested_rank: u32
        let requested_rank = crate::util::read_u32_le(r)?;

        Ok(Self {
            talent_id,
            requested_rank,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // talent_id: u32
        w.write_all(&self.talent_id.to_le_bytes())?;

        // requested_rank: u32
        w.write_all(&self.requested_rank.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for CMSG_LEARN_TALENT {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_LEARN_TALENT {
    fn maximum_possible_size() -> usize {
        4 // talent_id: u32
        + 4 // requested_rank: u32
    }
}

