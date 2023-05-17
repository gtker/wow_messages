use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_item_enchant_time_update.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_item_enchant_time_update.wowm#L3):
/// ```text
/// smsg SMSG_ITEM_ENCHANT_TIME_UPDATE = 0x01EB {
///     Guid item;
///     u32 slot;
///     u32 duration;
///     Guid player;
/// }
/// ```
pub struct SMSG_ITEM_ENCHANT_TIME_UPDATE {
    pub item: Guid,
    /// Possibly used with EnchantmentSlot enum.
    ///
    pub slot: u32,
    pub duration: u32,
    pub player: Guid,
}

impl crate::private::Sealed for SMSG_ITEM_ENCHANT_TIME_UPDATE {}
impl crate::Message for SMSG_ITEM_ENCHANT_TIME_UPDATE {
    const OPCODE: u32 = 0x01eb;

    fn size_without_header(&self) -> u32 {
        24
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // item: Guid
        w.write_all(&self.item.guid().to_le_bytes())?;

        // slot: u32
        w.write_all(&self.slot.to_le_bytes())?;

        // duration: u32
        w.write_all(&self.duration.to_le_bytes())?;

        // player: Guid
        w.write_all(&self.player.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 24 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01EB, size: body_size });
        }

        // item: Guid
        let item = Guid::read(&mut r)?;

        // slot: u32
        let slot = crate::util::read_u32_le(&mut r)?;

        // duration: u32
        let duration = crate::util::read_u32_le(&mut r)?;

        // player: Guid
        let player = Guid::read(&mut r)?;

        Ok(Self {
            item,
            slot,
            duration,
            player,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_ITEM_ENCHANT_TIME_UPDATE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_ITEM_ENCHANT_TIME_UPDATE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_ITEM_ENCHANT_TIME_UPDATE {}

