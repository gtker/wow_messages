use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ServerMessageWrite, MessageBody};
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
pub struct SMSG_AREA_SPIRIT_HEALER_TIME {
    pub guid: Guid,
    pub next_resurrect_time: u32,
}

impl ServerMessageWrite for SMSG_AREA_SPIRIT_HEALER_TIME {}

impl MessageBody for SMSG_AREA_SPIRIT_HEALER_TIME {
    const OPCODE: u16 = 0x02e4;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // next_resurrect_time: u32
        let next_resurrect_time = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            next_resurrect_time,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // next_resurrect_time: u32
        w.write_all(&self.next_resurrect_time.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_AREA_SPIRIT_HEALER_TIME {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_AREA_SPIRIT_HEALER_TIME {
    fn maximum_possible_size() -> usize {
        8 // guid: Guid
        + 4 // next_resurrect_time: u32
    }
}

