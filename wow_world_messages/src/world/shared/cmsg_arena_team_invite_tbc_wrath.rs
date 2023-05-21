use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/arena/cmsg_arena_team_invite.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/arena/cmsg_arena_team_invite.wowm#L1):
/// ```text
/// cmsg CMSG_ARENA_TEAM_INVITE = 0x034F {
///     u32 arena_team;
///     CString player;
/// }
/// ```
pub struct CMSG_ARENA_TEAM_INVITE {
    pub arena_team: u32,
    pub player: String,
}

impl crate::private::Sealed for CMSG_ARENA_TEAM_INVITE {}
impl CMSG_ARENA_TEAM_INVITE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(5..=260).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x034F, size: body_size });
        }

        // arena_team: u32
        let arena_team = crate::util::read_u32_le(&mut r)?;

        // player: CString
        let player = {
            let player = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(player)?
        };

        Ok(Self {
            arena_team,
            player,
        })
    }

}

impl crate::Message for CMSG_ARENA_TEAM_INVITE {
    const OPCODE: u32 = 0x034f;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_ARENA_TEAM_INVITE {{").unwrap();
        // Members
        writeln!(s, "    arena_team = {};", self.arena_team).unwrap();
        writeln!(s, "    player = \"{}\";", self.player).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 847_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "arena_team", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.player.len() + 1, "player", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // arena_team: u32
        w.write_all(&self.arena_team.to_le_bytes())?;

        // player: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.player.as_bytes().iter().rev().next(), Some(&0_u8), "String `player` must not be null-terminated.");
        w.write_all(self.player.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_ARENA_TEAM_INVITE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_ARENA_TEAM_INVITE {}

impl CMSG_ARENA_TEAM_INVITE {
    pub(crate) fn size(&self) -> usize {
        4 // arena_team: u32
        + self.player.len() + 1 // player: CString
    }
}

