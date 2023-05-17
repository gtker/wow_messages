use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/cmsg_item_text_query.wowm:13`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/cmsg_item_text_query.wowm#L13):
/// ```text
/// cmsg CMSG_ITEM_TEXT_QUERY = 0x0243 {
///     Guid item;
/// }
/// ```
pub struct CMSG_ITEM_TEXT_QUERY {
    pub item: Guid,
}

impl crate::private::Sealed for CMSG_ITEM_TEXT_QUERY {}
impl crate::Message for CMSG_ITEM_TEXT_QUERY {
    const OPCODE: u32 = 0x0243;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // item: Guid
        w.write_all(&self.item.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0243, size: body_size });
        }

        // item: Guid
        let item = Guid::read(&mut r)?;

        Ok(Self {
            item,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_ITEM_TEXT_QUERY {}

