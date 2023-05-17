use std::io::{Read, Write};

use crate::vanilla::ItemSlot;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_swap_inv_item.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_swap_inv_item.wowm#L8):
/// ```text
/// cmsg CMSG_SWAP_INV_ITEM = 0x010D {
///     ItemSlot source_slot;
///     ItemSlot destination_slot;
/// }
/// ```
pub struct CMSG_SWAP_INV_ITEM {
    pub source_slot: ItemSlot,
    pub destination_slot: ItemSlot,
}

impl crate::private::Sealed for CMSG_SWAP_INV_ITEM {}
impl crate::Message for CMSG_SWAP_INV_ITEM {
    const OPCODE: u32 = 0x010d;

    fn size_without_header(&self) -> u32 {
        2
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // source_slot: ItemSlot
        w.write_all(&(self.source_slot.as_int().to_le_bytes()))?;

        // destination_slot: ItemSlot
        w.write_all(&(self.destination_slot.as_int().to_le_bytes()))?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 2 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x010D, size: body_size });
        }

        // source_slot: ItemSlot
        let source_slot: ItemSlot = crate::util::read_u8_le(&mut r)?.try_into()?;

        // destination_slot: ItemSlot
        let destination_slot: ItemSlot = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            source_slot,
            destination_slot,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_SWAP_INV_ITEM {}

