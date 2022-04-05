use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/remaining.wowm:4046`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/remaining.wowm#L4046):
/// ```text
/// smsg SMSG_LOOT_ALL_PASSED = 0x29E {
///     u64 looted_target_guid;
///     u32 loot_slot;
///     u32 item_id;
///     u32 item_random_property_id;
///     u32 item_random_suffix_id;
/// }
/// ```
pub struct SMSG_LOOT_ALL_PASSED {
    pub looted_target_guid: u64,
    pub loot_slot: u32,
    pub item_id: u32,
    pub item_random_property_id: u32,
    pub item_random_suffix_id: u32,
}

impl WorldServerMessageWrite for SMSG_LOOT_ALL_PASSED {
    const OPCODE: u16 = 0x29e;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (Self::size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (Self::size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_LOOT_ALL_PASSED {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // looted_target_guid: u64
        let looted_target_guid = crate::util::read_u64_le(r)?;

        // loot_slot: u32
        let loot_slot = crate::util::read_u32_le(r)?;

        // item_id: u32
        let item_id = crate::util::read_u32_le(r)?;

        // item_random_property_id: u32
        let item_random_property_id = crate::util::read_u32_le(r)?;

        // item_random_suffix_id: u32
        let item_random_suffix_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            looted_target_guid,
            loot_slot,
            item_id,
            item_random_property_id,
            item_random_suffix_id,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // looted_target_guid: u64
        w.write_all(&self.looted_target_guid.to_le_bytes())?;

        // loot_slot: u32
        w.write_all(&self.loot_slot.to_le_bytes())?;

        // item_id: u32
        w.write_all(&self.item_id.to_le_bytes())?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes())?;

        // item_random_suffix_id: u32
        w.write_all(&self.item_random_suffix_id.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_LOOT_ALL_PASSED {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_LOOT_ALL_PASSED {
    fn maximum_possible_size() -> usize {
        8 // looted_target_guid: u64
        + 4 // loot_slot: u32
        + 4 // item_id: u32
        + 4 // item_random_property_id: u32
        + 4 // item_random_suffix_id: u32
    }
}

