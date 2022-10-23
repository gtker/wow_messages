use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_buy_item.wowm:12`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_buy_item.wowm#L12):
/// ```text
/// cmsg CMSG_BUY_ITEM = 0x01A2 {
///     Guid vendor_guid;
///     u32 item_id;
///     u32 slot;
///     u8 amount;
/// }
/// ```
pub struct CMSG_BUY_ITEM {
    pub vendor_guid: Guid,
    pub item_id: u32,
    pub slot: u32,
    pub amount: u8,
}

impl crate::Message for CMSG_BUY_ITEM {
    const OPCODE: u32 = 0x01a2;

    fn size_without_header(&self) -> u32 {
        17
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // vendor_guid: Guid
        w.write_all(&self.vendor_guid.guid().to_le_bytes())?;

        // item_id: u32
        w.write_all(&self.item_id.to_le_bytes())?;

        // slot: u32
        w.write_all(&self.slot.to_le_bytes())?;

        // amount: u8
        w.write_all(&self.amount.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 17 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // vendor_guid: Guid
        let vendor_guid = Guid::read(r)?;

        // item_id: u32
        let item_id = crate::util::read_u32_le(r)?;

        // slot: u32
        let slot = crate::util::read_u32_le(r)?;

        // amount: u8
        let amount = crate::util::read_u8_le(r)?;

        Ok(Self {
            vendor_guid,
            item_id,
            slot,
            amount,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_BUY_ITEM {}

