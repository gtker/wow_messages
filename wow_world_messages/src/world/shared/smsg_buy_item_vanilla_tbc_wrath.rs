use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_buy_item.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_buy_item.wowm#L3):
/// ```text
/// smsg SMSG_BUY_ITEM = 0x01A4 {
///     Guid guid;
///     u32 vendor_slot;
///     u32 amount_for_sale;
///     u32 amount_bought;
/// }
/// ```
pub struct SMSG_BUY_ITEM {
    pub guid: Guid,
    /// Starts at index 1.
    /// arcemu has this field as milliseconds since something instead.
    ///
    pub vendor_slot: u32,
    pub amount_for_sale: u32,
    pub amount_bought: u32,
}

impl crate::Message for SMSG_BUY_ITEM {
    const OPCODE: u32 = 0x01a4;

    fn size_without_header(&self) -> u32 {
        20
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // vendor_slot: u32
        w.write_all(&self.vendor_slot.to_le_bytes())?;

        // amount_for_sale: u32
        w.write_all(&self.amount_for_sale.to_le_bytes())?;

        // amount_bought: u32
        w.write_all(&self.amount_bought.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 20 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01A4, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        // vendor_slot: u32
        let vendor_slot = crate::util::read_u32_le(r)?;

        // amount_for_sale: u32
        let amount_for_sale = crate::util::read_u32_le(r)?;

        // amount_bought: u32
        let amount_bought = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            vendor_slot,
            amount_for_sale,
            amount_bought,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_BUY_ITEM {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_BUY_ITEM {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_BUY_ITEM {}

