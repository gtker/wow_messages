use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_list_inventory.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_list_inventory.wowm#L3):
/// ```text
/// cmsg CMSG_LIST_INVENTORY = 0x019E {
///     Guid guid;
/// }
/// ```
pub struct CMSG_LIST_INVENTORY {
    pub guid: Guid,
}

impl crate::private::Sealed for CMSG_LIST_INVENTORY {}
impl crate::Message for CMSG_LIST_INVENTORY {
    const OPCODE: u32 = 0x019e;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x019E, size: body_size });
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        Ok(Self {
            guid,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_LIST_INVENTORY {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_LIST_INVENTORY {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_LIST_INVENTORY {}

