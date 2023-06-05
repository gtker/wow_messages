use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/msg_query_guild_bank_text.wowm:7`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/msg_query_guild_bank_text.wowm#L7):
/// ```text
/// cmsg MSG_QUERY_GUILD_BANK_TEXT_Client = 0x040A {
///     u8 tab;
/// }
/// ```
pub struct MSG_QUERY_GUILD_BANK_TEXT_Client {
    pub tab: u8,
}

#[cfg(feature = "print-testcase")]
impl MSG_QUERY_GUILD_BANK_TEXT_Client {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test MSG_QUERY_GUILD_BANK_TEXT_Client {{").unwrap();
        // Members
        writeln!(s, "    tab = {};", self.tab).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 5_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1034_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "tab", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for MSG_QUERY_GUILD_BANK_TEXT_Client {}
impl crate::Message for MSG_QUERY_GUILD_BANK_TEXT_Client {
    const OPCODE: u32 = 0x040a;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        MSG_QUERY_GUILD_BANK_TEXT_Client::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // tab: u8
        w.write_all(&self.tab.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x040A, size: body_size });
        }

        // tab: u8
        let tab = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            tab,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for MSG_QUERY_GUILD_BANK_TEXT_Client {}

