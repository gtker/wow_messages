use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_item_text_query_response.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_item_text_query_response.wowm#L1):
/// ```text
/// smsg SMSG_ITEM_TEXT_QUERY_RESPONSE = 0x0244 {
///     u32 item_text_id;
///     CString text;
/// }
/// ```
pub struct SMSG_ITEM_TEXT_QUERY_RESPONSE {
    pub item_text_id: u32,
    /// mangoszero: CString TODO max length 8000
    ///
    pub text: String,
}

#[cfg(feature = "print-testcase")]
impl SMSG_ITEM_TEXT_QUERY_RESPONSE {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_ITEM_TEXT_QUERY_RESPONSE {{").unwrap();
        // Members
        writeln!(s, "    item_text_id = {};", self.item_text_id).unwrap();
        writeln!(s, "    text = \"{}\";", self.text).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 580_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "item_text_id");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1 2\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_ITEM_TEXT_QUERY_RESPONSE {}
impl crate::Message for SMSG_ITEM_TEXT_QUERY_RESPONSE {
    const OPCODE: u32 = 0x0244;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // item_text_id: u32
        w.write_all(&self.item_text_id.to_le_bytes())?;

        // text: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.text.as_bytes().iter().rev().next(), Some(&0_u8), "String `text` must not be null-terminated.");
        w.write_all(self.text.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(5..=260).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0244, size: body_size });
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

