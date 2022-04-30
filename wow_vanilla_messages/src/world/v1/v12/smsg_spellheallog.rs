use std::convert::{TryFrom, TryInto};
use crate::Guid;
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
pub struct SMSG_SPELLHEALLOG {
    pub victim_guid: Guid,
    pub caster_guid: Guid,
    pub id: u32,
    pub damage: u32,
    pub critical: u8,
}

impl ServerMessageWrite for SMSG_SPELLHEALLOG {
    const OPCODE: u16 = 0x150;

    fn size_without_size_field(&self) -> u16 {
        self.size() as u16
    }

}

impl MessageBody for SMSG_SPELLHEALLOG {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // victim_guid: PackedGuid
        let victim_guid = Guid::read_packed(r)?;

        // caster_guid: PackedGuid
        let caster_guid = Guid::read_packed(r)?;

        // id: u32
        let id = crate::util::read_u32_le(r)?;

        // damage: u32
        let damage = crate::util::read_u32_le(r)?;

        // critical: u8
        let critical = crate::util::read_u8_le(r)?;

        Ok(Self {
            victim_guid,
            caster_guid,
            id,
            damage,
            critical,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // victim_guid: PackedGuid
        self.victim_guid.write_packed(w)?;

        // caster_guid: PackedGuid
        self.caster_guid.write_packed(w)?;

        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // damage: u32
        w.write_all(&self.damage.to_le_bytes())?;

        // critical: u8
        w.write_all(&self.critical.to_le_bytes())?;

        Ok(())
    }
}

impl VariableSized for SMSG_SPELLHEALLOG {
    fn size(&self) -> usize {
        self.victim_guid.size() // victim_guid: PackedGuid
        + self.caster_guid.size() // caster_guid: PackedGuid
        + 4 // id: u32
        + 4 // damage: u32
        + 1 // critical: u8
    }
}

impl MaximumPossibleSized for SMSG_SPELLHEALLOG {
    fn maximum_possible_size() -> usize {
        9 // victim_guid: PackedGuid
        + 9 // caster_guid: PackedGuid
        + 4 // id: u32
        + 4 // damage: u32
        + 1 // critical: u8
    }
}

