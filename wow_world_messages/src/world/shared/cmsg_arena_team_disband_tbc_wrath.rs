use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/arena/cmsg_arena_team_disband.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/arena/cmsg_arena_team_disband.wowm#L1):
/// ```text
/// cmsg CMSG_ARENA_TEAM_DISBAND = 0x0355 {
///     u32 arena_team;
/// }
/// ```
pub struct CMSG_ARENA_TEAM_DISBAND {
    pub arena_team: u32,
}

impl crate::private::Sealed for CMSG_ARENA_TEAM_DISBAND {}
impl CMSG_ARENA_TEAM_DISBAND {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 4 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // arena_team: u32
        let arena_team = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            arena_team,
        })
    }

}

impl crate::Message for CMSG_ARENA_TEAM_DISBAND {
    const OPCODE: u32 = 0x0355;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_ARENA_TEAM_DISBAND {{").unwrap();
        // Members
        writeln!(s, "    arena_team = {};", self.arena_team).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 8_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 853_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "arena_team", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // arena_team: u32
        w.write_all(&self.arena_team.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(853, "CMSG_ARENA_TEAM_DISBAND", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_ARENA_TEAM_DISBAND {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_ARENA_TEAM_DISBAND {}

