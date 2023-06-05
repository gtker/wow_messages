use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/msg_query_guild_bank_text.wowm:20`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/msg_query_guild_bank_text.wowm#L20):
/// ```text
/// smsg MSG_QUERY_GUILD_BANK_TEXT_Server = 0x040A {
///     u8 tab;
///     CString text;
/// }
/// ```
pub struct MSG_QUERY_GUILD_BANK_TEXT_Server {
    pub tab: u8,
    pub text: String,
}

#[cfg(feature = "print-testcase")]
impl MSG_QUERY_GUILD_BANK_TEXT_Server {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test MSG_QUERY_GUILD_BANK_TEXT_Server {{").unwrap();
        // Members
        writeln!(s, "    tab = {};", self.tab).unwrap();
        writeln!(s, "    text = \"{}\";", self.text).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1034_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "tab");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for MSG_QUERY_GUILD_BANK_TEXT_Server {}
impl crate::Message for MSG_QUERY_GUILD_BANK_TEXT_Server {
    const OPCODE: u32 = 0x040a;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // tab: u8
        w.write_all(&self.tab.to_le_bytes())?;

        // text: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.text.as_bytes().iter().rev().next(), Some(&0_u8), "String `text` must not be null-terminated.");
        w.write_all(self.text.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(2..=257).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x040A, size: body_size });
        }

        // tab: u8
        let tab = crate::util::read_u8_le(&mut r)?;

        // text: CString
        let text = {
            let text = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(text)?
        };

        Ok(Self {
            tab,
            text,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_QUERY_GUILD_BANK_TEXT_Server {}

impl MSG_QUERY_GUILD_BANK_TEXT_Server {
    pub(crate) fn size(&self) -> usize {
        1 // tab: u8
        + self.text.len() + 1 // text: CString
    }
}

