use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/smsg_loot_start_roll.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/smsg_loot_start_roll.wowm#L1):
/// ```text
/// smsg SMSG_LOOT_START_ROLL = 0x02A1 {
///     Guid creature;
///     u32 loot_slot;
///     u32 item;
///     u32 item_random_suffix;
///     u32 item_random_property_id;
///     u32 countdown_time_in_milliseconds;
/// }
/// ```
pub struct SMSG_LOOT_START_ROLL {
    pub creature: Guid,
    pub loot_slot: u32,
    pub item: u32,
    /// vmangos/mangoszero: not used ?
    ///
    pub item_random_suffix: u32,
    pub item_random_property_id: u32,
    pub countdown_time_in_milliseconds: u32,
}

impl crate::Message for SMSG_LOOT_START_ROLL {
    const OPCODE: u32 = 0x02a1;

    fn size_without_header(&self) -> u32 {
        28
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // creature: Guid
        w.write_all(&self.creature.guid().to_le_bytes())?;

        // loot_slot: u32
        w.write_all(&self.loot_slot.to_le_bytes())?;

        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        // item_random_suffix: u32
        w.write_all(&self.item_random_suffix.to_le_bytes())?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes())?;

        // countdown_time_in_milliseconds: u32
        w.write_all(&self.countdown_time_in_milliseconds.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 28 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02A1, size: body_size as u32 });
        }

        // creature: Guid
        let creature = Guid::read(r)?;

        // loot_slot: u32
        let loot_slot = crate::util::read_u32_le(r)?;

        // item: u32
        let item = crate::util::read_u32_le(r)?;

        // item_random_suffix: u32
        let item_random_suffix = crate::util::read_u32_le(r)?;

        // item_random_property_id: u32
        let item_random_property_id = crate::util::read_u32_le(r)?;

        // countdown_time_in_milliseconds: u32
        let countdown_time_in_milliseconds = crate::util::read_u32_le(r)?;

        Ok(Self {
            creature,
            loot_slot,
            item,
            item_random_suffix,
            item_random_property_id,
            countdown_time_in_milliseconds,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_LOOT_START_ROLL {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_LOOT_START_ROLL {}

