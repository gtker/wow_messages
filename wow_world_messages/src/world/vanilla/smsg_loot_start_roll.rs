use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/smsg_loot_start_roll.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/smsg_loot_start_roll.wowm#L3):
/// ```text
/// smsg SMSG_LOOT_START_ROLL = 0x02A1 {
///     Guid creature_guid;
///     u32 loot_slot;
///     u32 item_id;
///     u32 item_random_suffix;
///     u32 item_random_property_id;
///     u32 countdown_time;
/// }
/// ```
pub struct SMSG_LOOT_START_ROLL {
    pub creature_guid: Guid,
    pub loot_slot: u32,
    pub item_id: u32,
    /// vmangos/mangoszero: not used ?
    ///
    pub item_random_suffix: u32,
    pub item_random_property_id: u32,
    pub countdown_time: u32,
}

impl crate::Message for SMSG_LOOT_START_ROLL {
    const OPCODE: u32 = 0x02a1;

    fn size_without_header(&self) -> u32 {
        28
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // creature_guid: Guid
        w.write_all(&self.creature_guid.guid().to_le_bytes())?;

        // loot_slot: u32
        w.write_all(&self.loot_slot.to_le_bytes())?;

        // item_id: u32
        w.write_all(&self.item_id.to_le_bytes())?;

        // item_random_suffix: u32
        w.write_all(&self.item_random_suffix.to_le_bytes())?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes())?;

        // countdown_time: u32
        w.write_all(&self.countdown_time.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 28 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // creature_guid: Guid
        let creature_guid = Guid::read(r)?;

        // loot_slot: u32
        let loot_slot = crate::util::read_u32_le(r)?;

        // item_id: u32
        let item_id = crate::util::read_u32_le(r)?;

        // item_random_suffix: u32
        let item_random_suffix = crate::util::read_u32_le(r)?;

        // item_random_property_id: u32
        let item_random_property_id = crate::util::read_u32_le(r)?;

        // countdown_time: u32
        let countdown_time = crate::util::read_u32_le(r)?;

        Ok(Self {
            creature_guid,
            loot_slot,
            item_id,
            item_random_suffix,
            item_random_property_id,
            countdown_time,
        })
    }

}
impl ServerMessage for SMSG_LOOT_START_ROLL {}

