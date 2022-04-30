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
pub struct CMSG_DESTROYITEM {
    pub bag: u8,
    pub slot: u8,
    pub amount: u8,
    pub unknown1: u8,
    pub unknown2: u8,
    pub unknown3: u8,
}

impl ClientMessageWrite for CMSG_DESTROYITEM {}

impl MessageBody for CMSG_DESTROYITEM {
    const OPCODE: u16 = 0x0111;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // bag: u8
        let bag = crate::util::read_u8_le(r)?;

        // slot: u8
        let slot = crate::util::read_u8_le(r)?;

        // amount: u8
        let amount = crate::util::read_u8_le(r)?;

        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(r)?;

        // unknown2: u8
        let unknown2 = crate::util::read_u8_le(r)?;

        // unknown3: u8
        let unknown3 = crate::util::read_u8_le(r)?;

        Ok(Self {
            bag,
            slot,
            amount,
            unknown1,
            unknown2,
            unknown3,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // bag: u8
        w.write_all(&self.bag.to_le_bytes())?;

        // slot: u8
        w.write_all(&self.slot.to_le_bytes())?;

        // amount: u8
        w.write_all(&self.amount.to_le_bytes())?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: u8
        w.write_all(&self.unknown2.to_le_bytes())?;

        // unknown3: u8
        w.write_all(&self.unknown3.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for CMSG_DESTROYITEM {}

impl MaximumPossibleSized for CMSG_DESTROYITEM {
    fn maximum_possible_size() -> usize {
        1 // bag: u8
        + 1 // slot: u8
        + 1 // amount: u8
        + 1 // unknown1: u8
        + 1 // unknown2: u8
        + 1 // unknown3: u8
    }
}

