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
pub struct SMSG_ATTACKERSTATEUPDATE {
    pub hit_info: u32,
    pub attacker: Guid,
    pub target: Guid,
    pub total_damage: u32,
}

impl ServerMessageWrite for SMSG_ATTACKERSTATEUPDATE {}

impl MessageBody for SMSG_ATTACKERSTATEUPDATE {
    const OPCODE: u16 = 0x014a;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // hit_info: u32
        let hit_info = crate::util::read_u32_le(r)?;

        // attacker: PackedGuid
        let attacker = Guid::read_packed(r)?;

        // target: PackedGuid
        let target = Guid::read_packed(r)?;

        // total_damage: u32
        let total_damage = crate::util::read_u32_le(r)?;

        Ok(Self {
            hit_info,
            attacker,
            target,
            total_damage,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // hit_info: u32
        w.write_all(&self.hit_info.to_le_bytes())?;

        // attacker: PackedGuid
        self.attacker.write_packed(w)?;

        // target: PackedGuid
        self.target.write_packed(w)?;

        // total_damage: u32
        w.write_all(&self.total_damage.to_le_bytes())?;

        Ok(())
    }
}

impl VariableSized for SMSG_ATTACKERSTATEUPDATE {
    fn size(&self) -> usize {
        4 // hit_info: u32
        + self.attacker.size() // attacker: PackedGuid
        + self.target.size() // target: PackedGuid
        + 4 // total_damage: u32
    }
}

impl MaximumPossibleSized for SMSG_ATTACKERSTATEUPDATE {
    fn maximum_possible_size() -> usize {
        4 // hit_info: u32
        + 9 // attacker: PackedGuid
        + 9 // target: PackedGuid
        + 4 // total_damage: u32
    }
}

