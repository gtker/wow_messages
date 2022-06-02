use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::version_1_12::RollVote;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/smsg_loot_roll.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/smsg_loot_roll.wowm#L3):
/// ```text
/// smsg SMSG_LOOT_ROLL = 0x02A2 {
///     Guid creature_guid;
///     u32 loot_slot;
///     Guid item_guid;
///     u32 item_id;
///     u32 item_random_suffix;
///     u32 item_random_property_id;
///     u8 roll_number;
///     RollVote vote;
/// }
/// ```
pub struct SMSG_LOOT_ROLL {
    pub creature_guid: Guid,
    pub loot_slot: u32,
    pub item_guid: Guid,
    pub item_id: u32,
    /// vmangos/mangoszero: not used ?
    ///
    pub item_random_suffix: u32,
    pub item_random_property_id: u32,
    /// vmangos/cmangos/mangoszero: 0: Need for: `item_name` > 127: you passed on: `item_name`      Roll number
    ///
    pub roll_number: u8,
    pub vote: RollVote,
}

impl ServerMessage for SMSG_LOOT_ROLL {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // creature_guid: Guid
        w.write_all(&self.creature_guid.guid().to_le_bytes())?;

        // loot_slot: u32
        w.write_all(&self.loot_slot.to_le_bytes())?;

        // item_guid: Guid
        w.write_all(&self.item_guid.guid().to_le_bytes())?;

        // item_id: u32
        w.write_all(&self.item_id.to_le_bytes())?;

        // item_random_suffix: u32
        w.write_all(&self.item_random_suffix.to_le_bytes())?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes())?;

        // roll_number: u8
        w.write_all(&self.roll_number.to_le_bytes())?;

        // vote: RollVote
        w.write_all(&(self.vote.as_int() as u8).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x02a2;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        34
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // creature_guid: Guid
        let creature_guid = Guid::read(r)?;

        // loot_slot: u32
        let loot_slot = crate::util::read_u32_le(r)?;

        // item_guid: Guid
        let item_guid = Guid::read(r)?;

        // item_id: u32
        let item_id = crate::util::read_u32_le(r)?;

        // item_random_suffix: u32
        let item_random_suffix = crate::util::read_u32_le(r)?;

        // item_random_property_id: u32
        let item_random_property_id = crate::util::read_u32_le(r)?;

        // roll_number: u8
        let roll_number = crate::util::read_u8_le(r)?;

        // vote: RollVote
        let vote: RollVote = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            creature_guid,
            loot_slot,
            item_guid,
            item_id,
            item_random_suffix,
            item_random_property_id,
            roll_number,
            vote,
        })
    }

}

