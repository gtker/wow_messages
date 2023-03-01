use crate:: {
    Guid,
};
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_item_refund.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_item_refund.wowm#L1):
/// ```text
/// cmsg CMSG_ITEM_REFUND = 0x04B4 {
///     Guid item;
/// }
/// ```
pub struct CMSG_ITEM_REFUND {
    pub item: Guid,
}

impl crate::Message for CMSG_ITEM_REFUND {
    const OPCODE: u32 = 0x04b4;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // item: Guid
        w.write_all(&self.item.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04B4, size: body_size as u32 });
        }

        // item: Guid
        let item = Guid::read(&mut r)?;

        Ok(Self {
            item,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_ITEM_REFUND {}

