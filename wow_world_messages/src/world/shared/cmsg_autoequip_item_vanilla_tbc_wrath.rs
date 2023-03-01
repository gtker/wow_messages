use crate:: {
};
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_autoequip_item.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_autoequip_item.wowm#L1):
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

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // source_bag: u8
        w.write_all(&self.source_bag.to_le_bytes())?;

        // source_slot: u8
        w.write_all(&self.source_slot.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 2 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x010A, size: body_size as u32 });
        }

        // source_bag: u8
        let source_bag = crate::util::read_u8_le(&mut r)?;

        // source_slot: u8
        let source_slot = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            source_bag,
            source_slot,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_AUTOEQUIP_ITEM {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_AUTOEQUIP_ITEM {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_AUTOEQUIP_ITEM {}

