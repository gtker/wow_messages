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
pub struct CMSG_LEAVE_BATTLEFIELD {
    pub unknown1: u8,
    pub battle_ground_type_id: u8,
    pub unknown2: u16,
}

impl ClientMessageWrite for CMSG_LEAVE_BATTLEFIELD {}

impl MessageBody for CMSG_LEAVE_BATTLEFIELD {
    const OPCODE: u16 = 0x02e1;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(r)?;

        // battle_ground_type_id: u8
        let battle_ground_type_id = crate::util::read_u8_le(r)?;

        // unknown2: u16
        let unknown2 = crate::util::read_u16_le(r)?;

        Ok(Self {
            unknown1,
            battle_ground_type_id,
            unknown2,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        // battle_ground_type_id: u8
        w.write_all(&self.battle_ground_type_id.to_le_bytes())?;

        // unknown2: u16
        w.write_all(&self.unknown2.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for CMSG_LEAVE_BATTLEFIELD {}

impl MaximumPossibleSized for CMSG_LEAVE_BATTLEFIELD {
    fn maximum_possible_size() -> usize {
        1 // unknown1: u8
        + 1 // battle_ground_type_id: u8
        + 2 // unknown2: u16
    }
}

