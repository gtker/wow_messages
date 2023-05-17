use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_page_text_query_response.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_page_text_query_response.wowm#L3):
/// ```text
/// smsg SMSG_PAGE_TEXT_QUERY_RESPONSE = 0x005B {
///     u32 page_id;
///     CString text;
///     u32 next_page_id;
/// }
/// ```
pub struct SMSG_PAGE_TEXT_QUERY_RESPONSE {
    pub page_id: u32,
    pub text: String,
    pub next_page_id: u32,
}

impl crate::private::Sealed for SMSG_PAGE_TEXT_QUERY_RESPONSE {}
impl crate::Message for SMSG_PAGE_TEXT_QUERY_RESPONSE {
    const OPCODE: u32 = 0x005b;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // page_id: u32
        w.write_all(&self.page_id.to_le_bytes())?;

        // text: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.text.as_bytes().iter().rev().next(), Some(&0_u8), "String `text` must not be null-terminated.");
        w.write_all(self.text.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // next_page_id: u32
        w.write_all(&self.next_page_id.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(9..=264).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x005B, size: body_size });
        }

        // page_id: u32
        let page_id = crate::util::read_u32_le(&mut r)?;

        // text: CString
        let text = {
            let text = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(text)?
        };

        // next_page_id: u32
        let next_page_id = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            page_id,
            text,
            next_page_id,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_PAGE_TEXT_QUERY_RESPONSE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_PAGE_TEXT_QUERY_RESPONSE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_PAGE_TEXT_QUERY_RESPONSE {}

impl SMSG_PAGE_TEXT_QUERY_RESPONSE {
    pub(crate) fn size(&self) -> usize {
        4 // page_id: u32
        + self.text.len() + 1 // text: CString
        + 4 // next_page_id: u32
    }
}

