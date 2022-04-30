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
pub struct SMSG_DISPEL_FAILED {
    pub caster_guid: Guid,
    pub target_guid: Guid,
    pub spells: Vec<u32>,
}

impl ServerMessageWrite for SMSG_DISPEL_FAILED {}

impl MessageBody for SMSG_DISPEL_FAILED {
    const OPCODE: u16 = 0x0262;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // caster_guid: Guid
        let caster_guid = Guid::read(r)?;

        // target_guid: Guid
        let target_guid = Guid::read(r)?;

        // spells: u32[-]
        let mut current_size = {
            8 // caster_guid: Guid
            + 8 // target_guid: Guid
        };
        let mut spells = Vec::with_capacity(body_size as usize - current_size);
        while current_size < (body_size as usize) {
            spells.push(crate::util::read_u32_le(r)?);
            current_size += 1;
        }

        Ok(Self {
            caster_guid,
            target_guid,
            spells,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // caster_guid: Guid
        self.caster_guid.write(w)?;

        // target_guid: Guid
        self.target_guid.write(w)?;

        // spells: u32[-]
        for i in self.spells.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
}

impl VariableSized for SMSG_DISPEL_FAILED {
    fn size(&self) -> usize {
        8 // caster_guid: Guid
        + 8 // target_guid: Guid
        + self.spells.len() * core::mem::size_of::<u32>() // spells: u32[-]
    }
}

impl MaximumPossibleSized for SMSG_DISPEL_FAILED {
    fn maximum_possible_size() -> usize {
        8 // caster_guid: Guid
        + 8 // target_guid: Guid
        + 65536 // spells: u32[-]
    }
}

