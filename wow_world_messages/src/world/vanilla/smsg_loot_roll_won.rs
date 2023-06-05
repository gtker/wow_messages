use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::RollVote;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/smsg_loot_roll_won.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/smsg_loot_roll_won.wowm#L1):
/// ```text
/// smsg SMSG_LOOT_ROLL_WON = 0x029F {
///     Guid looted_target;
///     u32 loot_slot;
///     u32 item;
///     u32 item_random_suffix;
///     u32 item_random_property_id;
///     Guid winning_player;
///     u8 winning_roll;
///     RollVote vote;
/// }
/// ```
pub struct SMSG_LOOT_ROLL_WON {
    pub looted_target: Guid,
    pub loot_slot: u32,
    pub item: u32,
    /// vmangos/mangoszero: not used ?
    ///
    pub item_random_suffix: u32,
    pub item_random_property_id: u32,
    pub winning_player: Guid,
    /// rollnumber related to SMSG_LOOT_ROLL
    ///
    pub winning_roll: u8,
    /// Rolltype related to SMSG_LOOT_ROLL
    ///
    pub vote: RollVote,
}

impl crate::private::Sealed for SMSG_LOOT_ROLL_WON {}
impl crate::Message for SMSG_LOOT_ROLL_WON {
    const OPCODE: u32 = 0x029f;

    fn size_without_header(&self) -> u32 {
        34
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // looted_target: Guid
        w.write_all(&self.looted_target.guid().to_le_bytes())?;

        // loot_slot: u32
        w.write_all(&self.loot_slot.to_le_bytes())?;

        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        // item_random_suffix: u32
        w.write_all(&self.item_random_suffix.to_le_bytes())?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes())?;

        // winning_player: Guid
        w.write_all(&self.winning_player.guid().to_le_bytes())?;

        // winning_roll: u8
        w.write_all(&self.winning_roll.to_le_bytes())?;

        // vote: RollVote
        w.write_all(&(self.vote.as_int().to_le_bytes()))?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 34 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x029F, size: body_size });
        }

        // looted_target: Guid
        let looted_target = crate::util::read_guid(&mut r)?;

        // loot_slot: u32
        let loot_slot = crate::util::read_u32_le(&mut r)?;

        // item: u32
        let item = crate::util::read_u32_le(&mut r)?;

        // item_random_suffix: u32
        let item_random_suffix = crate::util::read_u32_le(&mut r)?;

        // item_random_property_id: u32
        let item_random_property_id = crate::util::read_u32_le(&mut r)?;

        // winning_player: Guid
        let winning_player = crate::util::read_guid(&mut r)?;

        // winning_roll: u8
        let winning_roll = crate::util::read_u8_le(&mut r)?;

        // vote: RollVote
        let vote: RollVote = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            looted_target,
            loot_slot,
            item,
            item_random_suffix,
            item_random_property_id,
            winning_player,
            winning_roll,
            vote,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_LOOT_ROLL_WON {}

