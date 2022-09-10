use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_autoequip_item.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_autoequip_item.wowm#L3):
/// ```text
/// cmsg CMSG_AUTOEQUIP_ITEM = 0x010A {
///     u8 source_bag;
///     u8 source_slot;
/// }
/// ```
pub struct CMSG_AUTOEQUIP_ITEM {
    pub source_bag: u8,
    pub source_slot: u8,
}

impl crate::Message for CMSG_AUTOEQUIP_ITEM {
    const OPCODE: u32 = 0x010a;

    fn size_without_header(&self) -> u32 {
        2
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // source_bag: u8
        w.write_all(&self.source_bag.to_le_bytes())?;

        // source_slot: u8
        w.write_all(&self.source_slot.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 2 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // source_bag: u8
        let source_bag = crate::util::read_u8_le(r)?;

        // source_slot: u8
        let source_slot = crate::util::read_u8_le(r)?;

        Ok(Self {
            source_bag,
            source_slot,
        })
    }

}
#[cfg(any(feature = "vanilla", feature = "tbc"))]
impl crate::helper::shared::vanilla_tbc::ClientMessage for CMSG_AUTOEQUIP_ITEM {}

