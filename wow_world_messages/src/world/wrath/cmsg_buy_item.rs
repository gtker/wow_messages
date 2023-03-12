use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_buy_item.wowm:12`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_buy_item.wowm#L12):
/// ```text
/// cmsg CMSG_BUY_ITEM = 0x01A2 {
///     Guid vendor;
///     u32 item;
///     u32 slot;
///     u8 amount;
/// }
/// ```
pub struct CMSG_BUY_ITEM {
    pub vendor: Guid,
    pub item: u32,
    pub slot: u32,
    pub amount: u8,
}

impl crate::Message for CMSG_BUY_ITEM {
    const OPCODE: u32 = 0x01a2;

    fn size_without_header(&self) -> u32 {
        17
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // vendor: Guid
        w.write_all(&self.vendor.guid().to_le_bytes())?;

        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        // slot: u32
        w.write_all(&self.slot.to_le_bytes())?;

        // amount: u8
        w.write_all(&self.amount.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 17 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01A2, size: body_size as u32 });
        }

        // vendor: Guid
        let vendor = Guid::read(&mut r)?;

        // item: u32
        let item = crate::util::read_u32_le(&mut r)?;

        // slot: u32
        let slot = crate::util::read_u32_le(&mut r)?;

        // amount: u8
        let amount = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            vendor,
            item,
            slot,
            amount,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_BUY_ITEM {}

