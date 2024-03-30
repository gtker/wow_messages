use std::io::{Read, Write};

/// Reply to [`CMSG_QUERY_TIME`](crate::vanilla::CMSG_QUERY_TIME).
/// [`CMSG_QUERY_TIME`](crate::vanilla::CMSG_QUERY_TIME) and this reply does not actually appear to set the time. Instead [`SMSG_LOGIN_SETTIMESPEED`](crate::vanilla::SMSG_LOGIN_SETTIMESPEED) seems to correctly set the time. Running the client with `-console` will print the date when [`SMSG_LOGIN_SETTIMESPEED`](crate::vanilla::SMSG_LOGIN_SETTIMESPEED) is received, but not when this message is received.
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_query_time_response.wowm:22`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_query_time_response.wowm#L22):
/// ```text
/// smsg SMSG_QUERY_TIME_RESPONSE = 0x01CF {
///     u32 time;
///     u32 time_until_daily_quest_reset;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_QUERY_TIME_RESPONSE {
    /// Seconds since 1970, 1st of January (Unix Time).
    pub time: u32,
    /// Units need confirmation, but it's likely in seconds, since many other time related things are also seconds.
    pub time_until_daily_quest_reset: u32,
}

impl crate::private::Sealed for SMSG_QUERY_TIME_RESPONSE {}
impl SMSG_QUERY_TIME_RESPONSE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 8 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // time: u32
        let time = crate::util::read_u32_le(&mut r)?;

        // time_until_daily_quest_reset: u32
        let time_until_daily_quest_reset = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            time,
            time_until_daily_quest_reset,
        })
    }

}

impl crate::Message for SMSG_QUERY_TIME_RESPONSE {
    const OPCODE: u32 = 0x01cf;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_QUERY_TIME_RESPONSE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_QUERY_TIME_RESPONSE {{").unwrap();
        // Members
        writeln!(s, "    time = {};", self.time).unwrap();
        writeln!(s, "    time_until_daily_quest_reset = {};", self.time_until_daily_quest_reset).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 10_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 463_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "time", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_until_daily_quest_reset", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // time: u32
        w.write_all(&self.time.to_le_bytes())?;

        // time_until_daily_quest_reset: u32
        w.write_all(&self.time_until_daily_quest_reset.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(463, "SMSG_QUERY_TIME_RESPONSE", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_QUERY_TIME_RESPONSE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_QUERY_TIME_RESPONSE {}

