use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_autostore_bank_item.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_autostore_bank_item.wowm#L3):
/// ```text
/// cmsg CMSG_AUTOSTORE_BANK_ITEM = 0x0282 {
///     u8 bag_index;
///     u8 slot_index;
/// }
/// ```
pub struct CMSG_AUTOSTORE_BANK_ITEM {
    pub bag_index: u8,
    pub slot_index: u8,
}

impl crate::Message for CMSG_AUTOSTORE_BANK_ITEM {
    const OPCODE: u32 = 0x0282;

    fn size_without_header(&self) -> u32 {
        2
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // bag_index: u8
        w.write_all(&self.bag_index.to_le_bytes())?;

        // slot_index: u8
        w.write_all(&self.slot_index.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 2 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0282, size: body_size as u32 });
        }

        // bag_index: u8
        let bag_index = crate::util::read_u8_le(r)?;

        // slot_index: u8
        let slot_index = crate::util::read_u8_le(r)?;

        Ok(Self {
            bag_index,
            slot_index,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_AUTOSTORE_BANK_ITEM {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_AUTOSTORE_BANK_ITEM {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_AUTOSTORE_BANK_ITEM {}

