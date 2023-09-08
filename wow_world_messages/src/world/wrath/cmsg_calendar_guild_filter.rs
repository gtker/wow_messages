use std::io::{Read, Write};

use crate::shared::level_vanilla_tbc_wrath::Level;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/cmsg_calendar_guild_filter.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/cmsg_calendar_guild_filter.wowm#L1):
/// ```text
/// cmsg CMSG_CALENDAR_GUILD_FILTER = 0x042B {
///     Level32 minimum_level;
///     Level32 maximum_level;
///     u32 minimum_rank;
/// }
/// ```
pub struct CMSG_CALENDAR_GUILD_FILTER {
    pub minimum_level: Level,
    pub maximum_level: Level,
    pub minimum_rank: u32,
}

impl crate::private::Sealed for CMSG_CALENDAR_GUILD_FILTER {}
impl CMSG_CALENDAR_GUILD_FILTER {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 12 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // minimum_level: Level32
        let minimum_level = Level::new(crate::util::read_u32_le(&mut r)? as u8);

        // maximum_level: Level32
        let maximum_level = Level::new(crate::util::read_u32_le(&mut r)? as u8);

        // minimum_rank: u32
        let minimum_rank = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            minimum_level,
            maximum_level,
            minimum_rank,
        })
    }

}

impl crate::Message for CMSG_CALENDAR_GUILD_FILTER {
    const OPCODE: u32 = 0x042b;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_CALENDAR_GUILD_FILTER {{").unwrap();
        // Members
        writeln!(s, "    minimum_level = {};", self.minimum_level.as_int()).unwrap();
        writeln!(s, "    maximum_level = {};", self.maximum_level.as_int()).unwrap();
        writeln!(s, "    minimum_rank = {};", self.minimum_rank).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 16_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1067_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "minimum_level", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "maximum_level", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "minimum_rank", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // minimum_level: Level32
        w.write_all(&u32::from(self.minimum_level.as_int()).to_le_bytes())?;

        // maximum_level: Level32
        w.write_all(&u32::from(self.maximum_level.as_int()).to_le_bytes())?;

        // minimum_rank: u32
        w.write_all(&self.minimum_rank.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1067, "CMSG_CALENDAR_GUILD_FILTER", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_CALENDAR_GUILD_FILTER {}

