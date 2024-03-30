use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_page_text_query_response.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_page_text_query_response.wowm#L3):
/// ```text
/// smsg SMSG_PAGE_TEXT_QUERY_RESPONSE = 0x005B {
///     u32 page_id;
///     CString text;
///     u32 next_page_id;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PAGE_TEXT_QUERY_RESPONSE {
    pub page_id: u32,
    pub text: String,
    pub next_page_id: u32,
}

impl crate::private::Sealed for SMSG_PAGE_TEXT_QUERY_RESPONSE {}
impl SMSG_PAGE_TEXT_QUERY_RESPONSE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(9..=264).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
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

impl crate::Message for SMSG_PAGE_TEXT_QUERY_RESPONSE {
    const OPCODE: u32 = 0x005b;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_PAGE_TEXT_QUERY_RESPONSE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_PAGE_TEXT_QUERY_RESPONSE {{").unwrap();
        // Members
        writeln!(s, "    page_id = {};", self.page_id).unwrap();
        writeln!(s, "    text = \"{}\";", self.text).unwrap();
        writeln!(s, "    next_page_id = {};", self.next_page_id).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 91_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "page_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.text.len() + 1, "text", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "next_page_id", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // page_id: u32
        w.write_all(&self.page_id.to_le_bytes())?;

        // text: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.text.as_bytes().iter().next_back(), Some(&0_u8), "String `text` must not be null-terminated.");
        w.write_all(self.text.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // next_page_id: u32
        w.write_all(&self.next_page_id.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(91, "SMSG_PAGE_TEXT_QUERY_RESPONSE", body_size, a))
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

