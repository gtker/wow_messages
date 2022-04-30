use std::convert::{TryFrom, TryInto};
use crate::Guid;
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
pub struct CMSG_STABLE_SWAP_PET {
    pub npc: Guid,
    pub pet_slot: u32,
}

impl ClientMessageWrite for CMSG_STABLE_SWAP_PET {}

impl MessageBody for CMSG_STABLE_SWAP_PET {
    const OPCODE: u16 = 0x0275;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // npc: Guid
        let npc = Guid::read(r)?;

        // pet_slot: u32
        let pet_slot = crate::util::read_u32_le(r)?;

        Ok(Self {
            npc,
            pet_slot,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // npc: Guid
        self.npc.write(w)?;

        // pet_slot: u32
        w.write_all(&self.pet_slot.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for CMSG_STABLE_SWAP_PET {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_STABLE_SWAP_PET {
    fn maximum_possible_size() -> usize {
        8 // npc: Guid
        + 4 // pet_slot: u32
    }
}

