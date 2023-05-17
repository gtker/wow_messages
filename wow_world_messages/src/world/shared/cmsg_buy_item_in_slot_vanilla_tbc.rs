use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_buy_item_in_slot.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_buy_item_in_slot.wowm#L1):
/// ```text
/// cmsg CMSG_BUY_ITEM_IN_SLOT = 0x01A3 {
///     Guid vendor;
///     u32 item;
///     Guid bag;
///     u8 bag_slot;
///     u8 amount;
/// }
/// ```
pub struct CMSG_BUY_ITEM_IN_SLOT {
    pub vendor: Guid,
    pub item: u32,
    pub bag: Guid,
    pub bag_slot: u8,
    pub amount: u8,
}

impl crate::private::Sealed for CMSG_BUY_ITEM_IN_SLOT {}
impl crate::Message for CMSG_BUY_ITEM_IN_SLOT {
    const OPCODE: u32 = 0x01a3;

    fn size_without_header(&self) -> u32 {
        22
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // vendor: Guid
        w.write_all(&self.vendor.guid().to_le_bytes())?;

        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        // bag: Guid
        w.write_all(&self.bag.guid().to_le_bytes())?;

        // bag_slot: u8
        w.write_all(&self.bag_slot.to_le_bytes())?;

        // amount: u8
        w.write_all(&self.amount.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 22 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01A3, size: body_size });
        }

        // vendor: Guid
        let vendor = Guid::read(&mut r)?;

        // item: u32
        let item = crate::util::read_u32_le(&mut r)?;

        // bag: Guid
        let bag = Guid::read(&mut r)?;

        // bag_slot: u8
        let bag_slot = crate::util::read_u8_le(&mut r)?;

        // amount: u8
        let amount = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            vendor,
            item,
            bag,
            bag_slot,
            amount,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_BUY_ITEM_IN_SLOT {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_BUY_ITEM_IN_SLOT {}

