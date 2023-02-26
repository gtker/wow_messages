use crate::Guid;
use wow_world_base::shared::roll_vote_tbc_wrath::RollVote;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/smsg_loot_roll.wowm:19`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/smsg_loot_roll.wowm#L19):
/// ```text
/// smsg SMSG_LOOT_ROLL = 0x02A2 {
///     Guid creature;
///     u32 loot_slot;
///     Guid player;
///     u32 item;
///     u32 item_random_suffix;
///     u32 item_random_property_id;
///     u8 roll_number;
///     RollVote vote;
///     u8 auto_pass;
/// }
/// ```
pub struct SMSG_LOOT_ROLL {
    pub creature: Guid,
    pub loot_slot: u32,
    pub player: Guid,
    pub item: u32,
    /// vmangos/mangoszero: not used ?
    ///
    pub item_random_suffix: u32,
    pub item_random_property_id: u32,
    /// vmangos/cmangos/mangoszero: 0: Need for: `item_name` > 127: you passed on: `item_name`      Roll number
    ///
    pub roll_number: u8,
    pub vote: RollVote,
    /// mangosone/arcemu sets to 0.
    /// mangosone: auto pass on loot
    /// arcemu: possibly related to disenchanting of loot
    /// azerothcore: 1: 'You automatically passed on: %s because you cannot loot that item.' - Possibly used in need before greed
    ///
    pub auto_pass: u8,
}

impl crate::Message for SMSG_LOOT_ROLL {
    const OPCODE: u32 = 0x02a2;

    fn size_without_header(&self) -> u32 {
        35
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // creature: Guid
        w.write_all(&self.creature.guid().to_le_bytes())?;

        // loot_slot: u32
        w.write_all(&self.loot_slot.to_le_bytes())?;

        // player: Guid
        w.write_all(&self.player.guid().to_le_bytes())?;

        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        // item_random_suffix: u32
        w.write_all(&self.item_random_suffix.to_le_bytes())?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes())?;

        // roll_number: u8
        w.write_all(&self.roll_number.to_le_bytes())?;

        // vote: RollVote
        w.write_all(&(self.vote.as_int() as u8).to_le_bytes())?;

        // auto_pass: u8
        w.write_all(&self.auto_pass.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 35 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02A2, size: body_size as u32 });
        }

        // creature: Guid
        let creature = Guid::read(r)?;

        // loot_slot: u32
        let loot_slot = crate::util::read_u32_le(r)?;

        // player: Guid
        let player = Guid::read(r)?;

        // item: u32
        let item = crate::util::read_u32_le(r)?;

        // item_random_suffix: u32
        let item_random_suffix = crate::util::read_u32_le(r)?;

        // item_random_property_id: u32
        let item_random_property_id = crate::util::read_u32_le(r)?;

        // roll_number: u8
        let roll_number = crate::util::read_u8_le(r)?;

        // vote: RollVote
        let vote: RollVote = crate::util::read_u8_le(r)?.try_into()?;

        // auto_pass: u8
        let auto_pass = crate::util::read_u8_le(r)?;

        Ok(Self {
            creature,
            loot_slot,
            player,
            item,
            item_random_suffix,
            item_random_property_id,
            roll_number,
            vote,
            auto_pass,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_LOOT_ROLL {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_LOOT_ROLL {}

