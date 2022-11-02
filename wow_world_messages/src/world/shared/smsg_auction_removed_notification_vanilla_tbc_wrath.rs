use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/smsg/smsg_auction_removed_notification.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/smsg/smsg_auction_removed_notification.wowm#L3):
/// ```text
/// smsg SMSG_AUCTION_REMOVED_NOTIFICATION = 0x028D {
///     u32 item;
///     u32 item_template;
///     u32 random_property_id;
/// }
/// ```
pub struct SMSG_AUCTION_REMOVED_NOTIFICATION {
    pub item: u32,
    pub item_template: u32,
    pub random_property_id: u32,
}

impl crate::Message for SMSG_AUCTION_REMOVED_NOTIFICATION {
    const OPCODE: u32 = 0x028d;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        // item_template: u32
        w.write_all(&self.item_template.to_le_bytes())?;

        // random_property_id: u32
        w.write_all(&self.random_property_id.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x028D, size: body_size as u32 });
        }

        // item: u32
        let item = crate::util::read_u32_le(r)?;

        // item_template: u32
        let item_template = crate::util::read_u32_le(r)?;

        // random_property_id: u32
        let random_property_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            item,
            item_template,
            random_property_id,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_AUCTION_REMOVED_NOTIFICATION {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_AUCTION_REMOVED_NOTIFICATION {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_AUCTION_REMOVED_NOTIFICATION {}

