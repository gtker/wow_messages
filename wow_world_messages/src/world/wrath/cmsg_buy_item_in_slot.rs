use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_buy_item_in_slot.wowm:11`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_buy_item_in_slot.wowm#L11):
/// ```text
/// cmsg CMSG_BUY_ITEM_IN_SLOT = 0x01A3 {
///     Guid vendor;
///     u32 item;
///     u32 vendor_slot;
///     Guid bag;
///     u8 bag_slot;
///     u8 amount;
/// }
/// ```
pub struct CMSG_BUY_ITEM_IN_SLOT {
    pub vendor: Guid,
    pub item: u32,
    /// arcemu: VLack: 3.1.2 This is the slot's number on the vendor's panel, starts from 1
    ///
    pub vendor_slot: u32,
    pub bag: Guid,
    pub bag_slot: u8,
    pub amount: u8,
}

impl crate::Message for CMSG_BUY_ITEM_IN_SLOT {
    const OPCODE: u32 = 0x01a3;

    fn size_without_header(&self) -> u32 {
        26
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // vendor: Guid
        w.write_all(&self.vendor.guid().to_le_bytes())?;

        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        // vendor_slot: u32
        w.write_all(&self.vendor_slot.to_le_bytes())?;

        // bag: Guid
        w.write_all(&self.bag.guid().to_le_bytes())?;

        // bag_slot: u8
        w.write_all(&self.bag_slot.to_le_bytes())?;

        // amount: u8
        w.write_all(&self.amount.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 26 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01A3, size: body_size as u32 });
        }

        // vendor: Guid
        let vendor = Guid::read(r)?;

        // item: u32
        let item = crate::util::read_u32_le(r)?;

        // vendor_slot: u32
        let vendor_slot = crate::util::read_u32_le(r)?;

        // bag: Guid
        let bag = Guid::read(r)?;

        // bag_slot: u8
        let bag_slot = crate::util::read_u8_le(r)?;

        // amount: u8
        let amount = crate::util::read_u8_le(r)?;

        Ok(Self {
            vendor,
            item,
            vendor_slot,
            bag,
            bag_slot,
            amount,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_BUY_ITEM_IN_SLOT {}

