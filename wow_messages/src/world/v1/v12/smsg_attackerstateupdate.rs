use std::convert::{TryFrom, TryInto};
use crate::world::helper::Guid;
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new_all.wowm:1675`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new_all.wowm#L1675):
/// ```text
/// smsg SMSG_ATTACKERSTATEUPDATE = 0x14A {
///     u32 hit_info;
///     PackedGuid attacker;
///     PackedGuid target;
///     u32 total_damage;
/// }
/// ```
pub struct SMSG_ATTACKERSTATEUPDATE {
    pub hit_info: u32,
    pub attacker: Guid,
    pub target: Guid,
    pub total_damage: u32,
}

impl WorldServerMessageWrite for SMSG_ATTACKERSTATEUPDATE {
    const OPCODE: u16 = 0x14a;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (self.size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (self.size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_ATTACKERSTATEUPDATE {
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

