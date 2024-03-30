use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/arena/smsg_arena_team_invite.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/arena/smsg_arena_team_invite.wowm#L1):
/// ```text
/// smsg SMSG_ARENA_TEAM_INVITE = 0x0350 {
///     CString player_name;
///     CString team_name;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_ARENA_TEAM_INVITE {
    pub player_name: String,
    pub team_name: String,
}

impl crate::private::Sealed for SMSG_ARENA_TEAM_INVITE {}
impl SMSG_ARENA_TEAM_INVITE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(2..=512).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // player_name: CString
        let player_name = {
            let player_name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(player_name)?
        };

        // team_name: CString
        let team_name = {
            let team_name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(team_name)?
        };

        Ok(Self {
            player_name,
            team_name,
        })
    }

}

impl crate::Message for SMSG_ARENA_TEAM_INVITE {
    const OPCODE: u32 = 0x0350;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_ARENA_TEAM_INVITE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_ARENA_TEAM_INVITE {{").unwrap();
        // Members
        writeln!(s, "    player_name = \"{}\";", self.player_name).unwrap();
        writeln!(s, "    team_name = \"{}\";", self.team_name).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 848_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, self.player_name.len() + 1, "player_name", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.team_name.len() + 1, "team_name", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // player_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.player_name.as_bytes().iter().next_back(), Some(&0_u8), "String `player_name` must not be null-terminated.");
        w.write_all(self.player_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // team_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.team_name.as_bytes().iter().next_back(), Some(&0_u8), "String `team_name` must not be null-terminated.");
        w.write_all(self.team_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(848, "SMSG_ARENA_TEAM_INVITE", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_ARENA_TEAM_INVITE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_ARENA_TEAM_INVITE {}

impl SMSG_ARENA_TEAM_INVITE {
    pub(crate) fn size(&self) -> usize {
        self.player_name.len() + 1 // player_name: CString
        + self.team_name.len() + 1 // team_name: CString
    }
}

