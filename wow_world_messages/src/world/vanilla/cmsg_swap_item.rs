use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Copy, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_swap_item.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_swap_item.wowm#L3):
/// ```text
/// cmsg CMSG_SWAP_ITEM = 0x010C {
///     u8 destination_bag;
///     u8 destionation_slot;
///     u8 source_bag;
///     u8 source_slot;
/// }
/// ```
pub struct CMSG_SWAP_ITEM {
    pub destination_bag: u8,
    pub destionation_slot: u8,
    pub source_bag: u8,
    pub source_slot: u8,
}

impl crate::Message for CMSG_SWAP_ITEM {
    const OPCODE: u32 = 0x010c;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // destination_bag: u8
        w.write_all(&self.destination_bag.to_le_bytes())?;

        // destionation_slot: u8
        w.write_all(&self.destionation_slot.to_le_bytes())?;

        // source_bag: u8
        w.write_all(&self.source_bag.to_le_bytes())?;

        // source_slot: u8
        w.write_all(&self.source_slot.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // destination_bag: u8
        let destination_bag = crate::util::read_u8_le(r)?;

        // destionation_slot: u8
        let destionation_slot = crate::util::read_u8_le(r)?;

        // source_bag: u8
        let source_bag = crate::util::read_u8_le(r)?;

        // source_slot: u8
        let source_slot = crate::util::read_u8_le(r)?;

        Ok(Self {
            destination_bag,
            destionation_slot,
            source_bag,
            source_slot,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_SWAP_ITEM {}

