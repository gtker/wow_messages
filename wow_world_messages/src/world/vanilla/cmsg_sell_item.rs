use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_sell_item.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_sell_item.wowm#L3):
/// ```text
/// cmsg CMSG_SELL_ITEM = 0x01A0 {
///     Guid vendor_guid;
///     Guid item_guid;
///     u8 amount;
/// }
/// ```
pub struct CMSG_SELL_ITEM {
    pub vendor_guid: Guid,
    pub item_guid: Guid,
    pub amount: u8,
}

impl crate::Message for CMSG_SELL_ITEM {
    const OPCODE: u32 = 0x01a0;

    fn size_without_header(&self) -> u32 {
        17
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // vendor_guid: Guid
        w.write_all(&self.vendor_guid.guid().to_le_bytes())?;

        // item_guid: Guid
        w.write_all(&self.item_guid.guid().to_le_bytes())?;

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

        // item_guid: Guid
        let item_guid = Guid::read(r)?;

        // amount: u8
        let amount = crate::util::read_u8_le(r)?;

        Ok(Self {
            vendor_guid,
            item_guid,
            amount,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_SELL_ITEM {}

