use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/cmsg_page_text_query.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/cmsg_page_text_query.wowm#L1):
/// ```text
/// cmsg CMSG_PAGE_TEXT_QUERY = 0x005A {
///     u32 page_id;
/// }
/// ```
pub struct CMSG_PAGE_TEXT_QUERY {
    pub page_id: u32,
}

impl crate::private::Sealed for CMSG_PAGE_TEXT_QUERY {}
impl crate::Message for CMSG_PAGE_TEXT_QUERY {
    const OPCODE: u32 = 0x005a;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // page_id: u32
        w.write_all(&self.page_id.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x005A, size: body_size as u32 });
        }

        // page_id: u32
        let page_id = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            page_id,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_PAGE_TEXT_QUERY {}

