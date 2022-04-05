use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/smsg_item_enchant_time_update.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/smsg_item_enchant_time_update.wowm#L3):
/// ```text
/// smsg SMSG_ITEM_ENCHANT_TIME_UPDATE = 0x1EB {
///     u64 item_guid;
///     u32 slot;
///     u32 duration;
///     u64 player_guid;
/// }
/// ```
pub struct SMSG_ITEM_ENCHANT_TIME_UPDATE {
    pub item_guid: u64,
    pub slot: u32,
    pub duration: u32,
    pub player_guid: u64,
}

impl WorldServerMessageWrite for SMSG_ITEM_ENCHANT_TIME_UPDATE {
    const OPCODE: u16 = 0x1eb;

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
impl WorldMessageBody for SMSG_ITEM_ENCHANT_TIME_UPDATE {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // item_guid: u64
        let item_guid = crate::util::read_u64_le(r)?;

        // slot: u32
        let slot = crate::util::read_u32_le(r)?;

        // duration: u32
        let duration = crate::util::read_u32_le(r)?;

        // player_guid: u64
        let player_guid = crate::util::read_u64_le(r)?;

        Ok(Self {
            item_guid,
            slot,
            duration,
            player_guid,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // item_guid: u64
        w.write_all(&self.item_guid.to_le_bytes())?;

        // slot: u32
        w.write_all(&self.slot.to_le_bytes())?;

        // duration: u32
        w.write_all(&self.duration.to_le_bytes())?;

        // player_guid: u64
        w.write_all(&self.player_guid.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_ITEM_ENCHANT_TIME_UPDATE {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_ITEM_ENCHANT_TIME_UPDATE {
    fn maximum_possible_size() -> usize {
        8 // item_guid: u64
        + 4 // slot: u32
        + 4 // duration: u32
        + 8 // player_guid: u64
    }
}

