use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/msg_guild_event_log_query.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/msg_guild_event_log_query.wowm#L1):
/// ```text
/// cmsg MSG_GUILD_EVENT_LOG_QUERY_Client = 0x03FE {
/// }
/// ```
pub struct MSG_GUILD_EVENT_LOG_QUERY_Client {
}

impl crate::private::Sealed for MSG_GUILD_EVENT_LOG_QUERY_Client {}
impl MSG_GUILD_EVENT_LOG_QUERY_Client {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 0 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        Ok(Self {
        })
    }

}

impl crate::Message for MSG_GUILD_EVENT_LOG_QUERY_Client {
    const OPCODE: u32 = 0x03fe;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test MSG_GUILD_EVENT_LOG_QUERY_Client {{").unwrap();
        // Members

        writeln!(s, "}} [").unwrap();

        let [a, b] = 4_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1022_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();



        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1022, "MSG_GUILD_EVENT_LOG_QUERY_Client", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for MSG_GUILD_EVENT_LOG_QUERY_Client {}

