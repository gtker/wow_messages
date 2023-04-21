use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/smsg_loot_all_passed.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/smsg_loot_all_passed.wowm#L3):
/// ```text
/// smsg SMSG_LOOT_ALL_PASSED = 0x029E {
///     Guid looted_target;
///     u32 loot_slot;
///     u32 item;
///     u32 item_random_property_id;
///     u32 item_random_suffix_id;
/// }
/// ```
pub struct SMSG_LOOT_ALL_PASSED {
    pub looted_target: Guid,
    pub loot_slot: u32,
    pub item: u32,
    pub item_random_property_id: u32,
    /// vmangos/mangoszero: Always set to 0.
    ///
    pub item_random_suffix_id: u32,
}

impl crate::private::Sealed for SMSG_LOOT_ALL_PASSED {}
impl crate::Message for SMSG_LOOT_ALL_PASSED {
    const OPCODE: u32 = 0x029e;

    fn size_without_header(&self) -> u32 {
        24
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // looted_target: Guid
        w.write_all(&self.looted_target.guid().to_le_bytes())?;

        // loot_slot: u32
        w.write_all(&self.loot_slot.to_le_bytes())?;

        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes())?;

        // item_random_suffix_id: u32
        w.write_all(&self.item_random_suffix_id.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 24 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x029E, size: body_size as u32 });
        }

        // looted_target: Guid
        let looted_target = Guid::read(&mut r)?;

        // loot_slot: u32
        let loot_slot = crate::util::read_u32_le(&mut r)?;

        // item: u32
        let item = crate::util::read_u32_le(&mut r)?;

        // item_random_property_id: u32
        let item_random_property_id = crate::util::read_u32_le(&mut r)?;

        // item_random_suffix_id: u32
        let item_random_suffix_id = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            looted_target,
            loot_slot,
            item,
            item_random_property_id,
            item_random_suffix_id,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_LOOT_ALL_PASSED {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_LOOT_ALL_PASSED {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_LOOT_ALL_PASSED {}

