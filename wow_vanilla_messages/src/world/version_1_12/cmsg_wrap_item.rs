use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_wrap_item.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_wrap_item.wowm#L3):
/// ```text
/// cmsg CMSG_WRAP_ITEM = 0x01D3 {
///     u8 gift_bag_index;
///     u8 gift_slot;
///     u8 item_bag_index;
///     u8 item_slot;
/// }
/// ```
pub struct CMSG_WRAP_ITEM {
    pub gift_bag_index: u8,
    pub gift_slot: u8,
    pub item_bag_index: u8,
    pub item_slot: u8,
}

impl ClientMessage for CMSG_WRAP_ITEM {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // gift_bag_index: u8
        w.write_all(&self.gift_bag_index.to_le_bytes())?;

        // gift_slot: u8
        w.write_all(&self.gift_slot.to_le_bytes())?;

        // item_bag_index: u8
        w.write_all(&self.item_bag_index.to_le_bytes())?;

        // item_slot: u8
        w.write_all(&self.item_slot.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x01d3;

    fn client_size(&self) -> u16 {
        10
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // gift_bag_index: u8
        let gift_bag_index = crate::util::read_u8_le(r)?;

        // gift_slot: u8
        let gift_slot = crate::util::read_u8_le(r)?;

        // item_bag_index: u8
        let item_bag_index = crate::util::read_u8_le(r)?;

        // item_slot: u8
        let item_slot = crate::util::read_u8_le(r)?;

        Ok(Self {
            gift_bag_index,
            gift_slot,
            item_bag_index,
            item_slot,
        })
    }

}

