use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_item_text_query_response.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_item_text_query_response.wowm#L1):
/// ```text
/// smsg SMSG_ITEM_TEXT_QUERY_RESPONSE = 0x0244 {
///     u32 item_text_id;
///     CString text;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_ITEM_TEXT_QUERY_RESPONSE {
    pub item_text_id: u32,
    /// mangoszero: CString TODO max length 8000
    pub text: String,
}

impl crate::private::Sealed for SMSG_ITEM_TEXT_QUERY_RESPONSE {}
impl SMSG_ITEM_TEXT_QUERY_RESPONSE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(5..=260).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // item_text_id: u32
        let item_text_id = crate::util::read_u32_le(&mut r)?;

        // text: CString
        let text = {
            let text = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(text)?
        };

        Ok(Self {
            item_text_id,
            text,
        })
    }

}

impl crate::Message for SMSG_ITEM_TEXT_QUERY_RESPONSE {
    const OPCODE: u32 = 0x0244;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_ITEM_TEXT_QUERY_RESPONSE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_ITEM_TEXT_QUERY_RESPONSE {{").unwrap();
        // Members
        writeln!(s, "    item_text_id = {};", self.item_text_id).unwrap();
        writeln!(s, "    text = \"{}\";", self.text).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 580_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "item_text_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.text.len() + 1, "text", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // item_text_id: u32
        w.write_all(&self.item_text_id.to_le_bytes())?;

        // text: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.text.as_bytes().iter().next_back(), Some(&0_u8), "String `text` must not be null-terminated.");
        w.write_all(self.text.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(580, "SMSG_ITEM_TEXT_QUERY_RESPONSE", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_ITEM_TEXT_QUERY_RESPONSE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_ITEM_TEXT_QUERY_RESPONSE {}

impl SMSG_ITEM_TEXT_QUERY_RESPONSE {
    pub(crate) fn size(&self) -> usize {
        4 // item_text_id: u32
        + self.text.len() + 1 // text: CString
    }
}

