use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/msg_query_guild_bank_text.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/msg_query_guild_bank_text.wowm#L1):
/// ```text
/// cmsg MSG_QUERY_GUILD_BANK_TEXT_Client = 0x0409 {
///     u8 tab;
/// }
/// ```
pub struct MSG_QUERY_GUILD_BANK_TEXT_Client {
    pub tab: u8,
}

impl crate::private::Sealed for MSG_QUERY_GUILD_BANK_TEXT_Client {}
impl MSG_QUERY_GUILD_BANK_TEXT_Client {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 1 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // tab: u8
        let tab = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            tab,
        })
    }

}

impl crate::Message for MSG_QUERY_GUILD_BANK_TEXT_Client {
    const OPCODE: u32 = 0x0409;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test MSG_QUERY_GUILD_BANK_TEXT_Client {{").unwrap();
        // Members
        writeln!(s, "    tab = {};", self.tab).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 5_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1033_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "tab", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // tab: u8
        w.write_all(&self.tab.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1033, "MSG_QUERY_GUILD_BANK_TEXT_Client", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for MSG_QUERY_GUILD_BANK_TEXT_Client {}

