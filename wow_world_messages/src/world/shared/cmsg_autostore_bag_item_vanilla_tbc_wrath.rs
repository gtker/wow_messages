use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_autostore_bag_item.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_autostore_bag_item.wowm#L3):
/// ```text
/// cmsg CMSG_AUTOSTORE_BAG_ITEM = 0x010B {
///     u8 source_bag;
///     u8 source_slot;
///     u8 destination_bag;
/// }
/// ```
pub struct CMSG_AUTOSTORE_BAG_ITEM {
    pub source_bag: u8,
    pub source_slot: u8,
    pub destination_bag: u8,
}

impl crate::Message for CMSG_AUTOSTORE_BAG_ITEM {
    const OPCODE: u32 = 0x010b;

    fn size_without_header(&self) -> u32 {
        3
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // source_bag: u8
        w.write_all(&self.source_bag.to_le_bytes())?;

        // source_slot: u8
        w.write_all(&self.source_slot.to_le_bytes())?;

        // destination_bag: u8
        w.write_all(&self.destination_bag.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 3 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x010B, size: body_size as u32 });
        }

        // source_bag: u8
        let source_bag = crate::util::read_u8_le(r)?;

        // source_slot: u8
        let source_slot = crate::util::read_u8_le(r)?;

        // destination_bag: u8
        let destination_bag = crate::util::read_u8_le(r)?;

        Ok(Self {
            source_bag,
            source_slot,
            destination_bag,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_AUTOSTORE_BAG_ITEM {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_AUTOSTORE_BAG_ITEM {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_AUTOSTORE_BAG_ITEM {}

