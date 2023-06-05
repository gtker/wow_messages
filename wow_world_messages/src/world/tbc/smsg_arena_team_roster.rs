use std::io::{Read, Write};

use crate::tbc::{
    ArenaTeamMember, ArenaType,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/arena/smsg_arena_team_roster.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/arena/smsg_arena_team_roster.wowm#L1):
/// ```text
/// smsg SMSG_ARENA_TEAM_ROSTER = 0x034E {
///     u32 arena_team;
///     u32 amount_of_members;
///     ArenaType arena_type;
///     ArenaTeamMember[amount_of_members] members;
/// }
/// ```
pub struct SMSG_ARENA_TEAM_ROSTER {
    pub arena_team: u32,
    pub arena_type: ArenaType,
    pub members: Vec<ArenaTeamMember>,
}

#[cfg(feature = "print-testcase")]
impl SMSG_ARENA_TEAM_ROSTER {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_ARENA_TEAM_ROSTER {{").unwrap();
        // Members
        writeln!(s, "    arena_team = {};", self.arena_team).unwrap();
        writeln!(s, "    amount_of_members = {};", self.members.len()).unwrap();
        writeln!(s, "    arena_type = {};", self.arena_type.as_test_case_value()).unwrap();
        write!(s, "    members = [").unwrap();
        for v in self.members.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        guid = {};", v.guid.guid()).unwrap();
            writeln!(s, "        online = {};", if v.online { "TRUE" } else { "FALSE" }).unwrap();
            writeln!(s, "        name = \"{}\";", v.name).unwrap();
            writeln!(s, "        level = {};", v.level.as_int()).unwrap();
            writeln!(s, "        class = {};", v.class.as_test_case_value()).unwrap();
            writeln!(s, "        games_played_this_week = {};", v.games_played_this_week).unwrap();
            writeln!(s, "        wins_this_week = {};", v.wins_this_week).unwrap();
            writeln!(s, "        games_played_this_season = {};", v.games_played_this_season).unwrap();
            writeln!(s, "        wins_this_season = {};", v.wins_this_season).unwrap();
            writeln!(s, "        personal_rating = {};", v.personal_rating).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 846_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "arena_team", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_members", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "arena_type", "    ");
        if !self.members.is_empty() {
            writeln!(s, "    /* members: ArenaTeamMember[amount_of_members] start */").unwrap();
            for (i, v) in self.members.iter().enumerate() {
                writeln!(s, "    /* members: ArenaTeamMember[amount_of_members] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "online", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, v.name.len() + 1, "name", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "level", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "class", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "games_played_this_week", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "wins_this_week", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "games_played_this_season", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "wins_this_season", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "personal_rating", "        ");
                writeln!(s, "    /* members: ArenaTeamMember[amount_of_members] {i} end */").unwrap();
            }
            writeln!(s, "    /* members: ArenaTeamMember[amount_of_members] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"2.4.3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_ARENA_TEAM_ROSTER {}
impl crate::Message for SMSG_ARENA_TEAM_ROSTER {
    const OPCODE: u32 = 0x034e;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_ARENA_TEAM_ROSTER::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // arena_team: u32
        w.write_all(&self.arena_team.to_le_bytes())?;

        // amount_of_members: u32
        w.write_all(&(self.members.len() as u32).to_le_bytes())?;

        // arena_type: ArenaType
        w.write_all(&(self.arena_type.as_int().to_le_bytes()))?;

        // members: ArenaTeamMember[amount_of_members]
        for i in self.members.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(9..=65535).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x034E, size: body_size });
        }

        // arena_team: u32
        let arena_team = crate::util::read_u32_le(&mut r)?;

        // amount_of_members: u32
        let amount_of_members = crate::util::read_u32_le(&mut r)?;

        // arena_type: ArenaType
        let arena_type = crate::util::read_u8_le(&mut r)?.try_into()?;

        // members: ArenaTeamMember[amount_of_members]
        let members = {
            let mut members = Vec::with_capacity(amount_of_members as usize);
            for _ in 0..amount_of_members {
                members.push(ArenaTeamMember::read(&mut r)?);
            }
            members
        };

        Ok(Self {
            arena_team,
            arena_type,
            members,
        })
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_ARENA_TEAM_ROSTER {}

impl SMSG_ARENA_TEAM_ROSTER {
    pub(crate) fn size(&self) -> usize {
        4 // arena_team: u32
        + 4 // amount_of_members: u32
        + 1 // arena_type: ArenaType
        + self.members.iter().fold(0, |acc, x| acc + x.size()) // members: ArenaTeamMember[amount_of_members]
    }
}

