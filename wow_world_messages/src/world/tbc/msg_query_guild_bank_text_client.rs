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

#[cfg(feature = "print-testcase")]
impl MSG_QUERY_GUILD_BANK_TEXT_Client {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test MSG_QUERY_GUILD_BANK_TEXT_Client {{").unwrap();
        // Members
        writeln!(s, "    tab = {};", self.tab).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 7_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1033_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
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
        writeln!(s, "    versions = \"2.4.3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for MSG_QUERY_GUILD_BANK_TEXT_Client {}
impl crate::Message for MSG_QUERY_GUILD_BANK_TEXT_Client {
    const OPCODE: u32 = 0x0409;

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
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0409, size: body_size });
        }

        // tab: u8
        let tab = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            tab,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for MSG_QUERY_GUILD_BANK_TEXT_Client {}

