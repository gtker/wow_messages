use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_buy_item.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_buy_item.wowm#L1):
/// ```text
/// cmsg CMSG_BUY_ITEM = 0x01A2 {
///     Guid vendor;
///     u32 item;
///     u8 amount;
///     u8 unknown1;
/// }
/// ```
pub struct CMSG_BUY_ITEM {
    pub vendor: Guid,
    pub item: u32,
    pub amount: u8,
    /// cmangos says this is hardcoded to 1 in the TBC client.
    ///
    pub unknown1: u8,
}

impl crate::Message for CMSG_BUY_ITEM {
    const OPCODE: u32 = 0x01a2;

    fn size_without_header(&self) -> u32 {
        14
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // vendor: Guid
        w.write_all(&self.vendor.guid().to_le_bytes())?;

        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        // amount: u8
        w.write_all(&self.amount.to_le_bytes())?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 14 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01A2, size: body_size as u32 });
        }

        // vendor: Guid
        let vendor = Guid::read(r)?;

        // item: u32
        let item = crate::util::read_u32_le(r)?;

        // amount: u8
        let amount = crate::util::read_u8_le(r)?;

        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(r)?;

        Ok(Self {
            vendor,
            item,
            amount,
            unknown1,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_BUY_ITEM {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_BUY_ITEM {}

