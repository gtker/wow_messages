use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::ItemQuality;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/cmsg/cmsg_auction_list_items.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/cmsg/cmsg_auction_list_items.wowm#L1):
/// ```text
/// cmsg CMSG_AUCTION_LIST_ITEMS = 0x0258 {
///     Guid auctioneer_guid;
///     u32 list_start_item;
///     CString searched_name;
///     u8 minimum_level;
///     u8 maximum_level;
///     u32 auction_slot_id;
///     u32 auction_main_category;
///     u32 auction_sub_category;
///     ItemQuality auction_quality;
///     u8 usable;
/// }
/// ```
pub struct CMSG_AUCTION_LIST_ITEMS {
    pub auctioneer_guid: Guid,
    pub list_start_item: u32,
    pub searched_name: String,
    pub minimum_level: u8,
    pub maximum_level: u8,
    pub auction_slot_id: u32,
    pub auction_main_category: u32,
    pub auction_sub_category: u32,
    pub auction_quality: ItemQuality,
    pub usable: u8,
}

impl crate::Message for CMSG_AUCTION_LIST_ITEMS {
    const OPCODE: u32 = 0x0258;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // auctioneer_guid: Guid
        w.write_all(&self.auctioneer_guid.guid().to_le_bytes())?;

        // list_start_item: u32
        w.write_all(&self.list_start_item.to_le_bytes())?;

        // searched_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.searched_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `searched_name` must not be null-terminated.");
        w.write_all(self.searched_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // minimum_level: u8
        w.write_all(&self.minimum_level.to_le_bytes())?;

        // maximum_level: u8
        w.write_all(&self.maximum_level.to_le_bytes())?;

        // auction_slot_id: u32
        w.write_all(&self.auction_slot_id.to_le_bytes())?;

        // auction_main_category: u32
        w.write_all(&self.auction_main_category.to_le_bytes())?;

        // auction_sub_category: u32
        w.write_all(&self.auction_sub_category.to_le_bytes())?;

        // auction_quality: ItemQuality
        w.write_all(&(self.auction_quality.as_int() as u32).to_le_bytes())?;

        // usable: u8
        w.write_all(&self.usable.to_le_bytes())?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size < 32 || body_size > 287 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0258, size: body_size as u32 });
        }

        // auctioneer_guid: Guid
        let auctioneer_guid = Guid::read(r)?;

        // list_start_item: u32
        let list_start_item = crate::util::read_u32_le(r)?;

        // searched_name: CString
        let searched_name = crate::util::read_c_string_to_vec(r)?;
        let searched_name = String::from_utf8(searched_name)?;

        // minimum_level: u8
        let minimum_level = crate::util::read_u8_le(r)?;

        // maximum_level: u8
        let maximum_level = crate::util::read_u8_le(r)?;

        // auction_slot_id: u32
        let auction_slot_id = crate::util::read_u32_le(r)?;

        // auction_main_category: u32
        let auction_main_category = crate::util::read_u32_le(r)?;

        // auction_sub_category: u32
        let auction_sub_category = crate::util::read_u32_le(r)?;

        // auction_quality: ItemQuality
        let auction_quality: ItemQuality = (crate::util::read_u32_le(r)? as u8).try_into()?;

        // usable: u8
        let usable = crate::util::read_u8_le(r)?;

        Ok(Self {
            auctioneer_guid,
            list_start_item,
            searched_name,
            minimum_level,
            maximum_level,
            auction_slot_id,
            auction_main_category,
            auction_sub_category,
            auction_quality,
            usable,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_AUCTION_LIST_ITEMS {}

impl CMSG_AUCTION_LIST_ITEMS {
    pub(crate) fn size(&self) -> usize {
        8 // auctioneer_guid: Guid
        + 4 // list_start_item: u32
        + self.searched_name.len() + 1 // searched_name: CString
        + 1 // minimum_level: u8
        + 1 // maximum_level: u8
        + 4 // auction_slot_id: u32
        + 4 // auction_main_category: u32
        + 4 // auction_sub_category: u32
        + 4 // auction_quality: ItemQuality
        + 1 // usable: u8
    }
}

