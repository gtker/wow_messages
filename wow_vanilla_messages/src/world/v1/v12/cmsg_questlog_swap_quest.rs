use std::convert::{TryFrom, TryInto};
use crate::{WorldClientMessageWrite, MessageBody};
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
pub struct CMSG_QUESTLOG_SWAP_QUEST {
    pub slot1: u8,
    pub slot2: u8,
}

impl WorldClientMessageWrite for CMSG_QUESTLOG_SWAP_QUEST {
    const OPCODE: u32 = 0x193;

    fn write_unencrypted_client<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        crate::util::write_u16_be(w, (Self::size() + 4) as u16)?;
        crate::util::write_u32_le(w, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_client<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        e.write_encrypted_client_header(w, (Self::size() + 4) as u16, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl MessageBody for CMSG_QUESTLOG_SWAP_QUEST {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // slot1: u8
        let slot1 = crate::util::read_u8_le(r)?;

        // slot2: u8
        let slot2 = crate::util::read_u8_le(r)?;

        Ok(Self {
            slot1,
            slot2,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // slot1: u8
        w.write_all(&self.slot1.to_le_bytes())?;

        // slot2: u8
        w.write_all(&self.slot2.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for CMSG_QUESTLOG_SWAP_QUEST {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_QUESTLOG_SWAP_QUEST {
    fn maximum_possible_size() -> usize {
        1 // slot1: u8
        + 1 // slot2: u8
    }
}

