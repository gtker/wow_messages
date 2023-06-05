use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/arena/msg_inspect_arena_teams.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/arena/msg_inspect_arena_teams.wowm#L1):
/// ```text
/// smsg MSG_INSPECT_ARENA_TEAMS_Server = 0x0377 {
///     Guid player;
///     u8 slot;
///     u32 arena_team;
///     u32 rating;
///     u32 games_played_this_season;
///     u32 wins_this_season;
///     u32 total_games_played;
///     u32 personal_rating;
/// }
/// ```
pub struct MSG_INSPECT_ARENA_TEAMS_Server {
    pub player: Guid,
    pub slot: u8,
    pub arena_team: u32,
    pub rating: u32,
    pub games_played_this_season: u32,
    pub wins_this_season: u32,
    pub total_games_played: u32,
    pub personal_rating: u32,
}

#[cfg(feature = "print-testcase")]
impl MSG_INSPECT_ARENA_TEAMS_Server {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test MSG_INSPECT_ARENA_TEAMS_Server {{").unwrap();
        // Members
        writeln!(s, "    player = {};", self.player.guid()).unwrap();
        writeln!(s, "    slot = {};", self.slot).unwrap();
        writeln!(s, "    arena_team = {};", self.arena_team).unwrap();
        writeln!(s, "    rating = {};", self.rating).unwrap();
        writeln!(s, "    games_played_this_season = {};", self.games_played_this_season).unwrap();
        writeln!(s, "    wins_this_season = {};", self.wins_this_season).unwrap();
        writeln!(s, "    total_games_played = {};", self.total_games_played).unwrap();
        writeln!(s, "    personal_rating = {};", self.personal_rating).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 35_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 887_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "player", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "slot", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "arena_team", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "rating", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "games_played_this_season", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "wins_this_season", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "total_games_played", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "personal_rating", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"2.4.3 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for MSG_INSPECT_ARENA_TEAMS_Server {}
impl crate::Message for MSG_INSPECT_ARENA_TEAMS_Server {
    const OPCODE: u32 = 0x0377;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        MSG_INSPECT_ARENA_TEAMS_Server::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        33
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // player: Guid
        w.write_all(&self.player.guid().to_le_bytes())?;

        // slot: u8
        w.write_all(&self.slot.to_le_bytes())?;

        // arena_team: u32
        w.write_all(&self.arena_team.to_le_bytes())?;

        // rating: u32
        w.write_all(&self.rating.to_le_bytes())?;

        // games_played_this_season: u32
        w.write_all(&self.games_played_this_season.to_le_bytes())?;

        // wins_this_season: u32
        w.write_all(&self.wins_this_season.to_le_bytes())?;

        // total_games_played: u32
        w.write_all(&self.total_games_played.to_le_bytes())?;

        // personal_rating: u32
        w.write_all(&self.personal_rating.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 33 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0377, size: body_size });
        }

        // player: Guid
        let player = crate::util::read_guid(&mut r)?;

        // slot: u8
        let slot = crate::util::read_u8_le(&mut r)?;

        // arena_team: u32
        let arena_team = crate::util::read_u32_le(&mut r)?;

        // rating: u32
        let rating = crate::util::read_u32_le(&mut r)?;

        // games_played_this_season: u32
        let games_played_this_season = crate::util::read_u32_le(&mut r)?;

        // wins_this_season: u32
        let wins_this_season = crate::util::read_u32_le(&mut r)?;

        // total_games_played: u32
        let total_games_played = crate::util::read_u32_le(&mut r)?;

        // personal_rating: u32
        let personal_rating = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            player,
            slot,
            arena_team,
            rating,
            games_played_this_season,
            wins_this_season,
            total_games_played,
            personal_rating,
        })
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for MSG_INSPECT_ARENA_TEAMS_Server {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_INSPECT_ARENA_TEAMS_Server {}

