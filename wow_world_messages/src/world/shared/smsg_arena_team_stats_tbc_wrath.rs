use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/arena/smsg_arena_team_stats.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/arena/smsg_arena_team_stats.wowm#L1):
/// ```text
/// smsg SMSG_ARENA_TEAM_STATS = 0x035B {
///     u32 arena_team;
///     u32 rating;
///     u32 games_played_this_week;
///     u32 games_won_this_week;
///     u32 games_played_this_season;
///     u32 games_won_this_season;
///     u32 ranking;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_ARENA_TEAM_STATS {
    pub arena_team: u32,
    pub rating: u32,
    pub games_played_this_week: u32,
    pub games_won_this_week: u32,
    pub games_played_this_season: u32,
    pub games_won_this_season: u32,
    pub ranking: u32,
}

impl crate::private::Sealed for SMSG_ARENA_TEAM_STATS {}
impl SMSG_ARENA_TEAM_STATS {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 28 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // arena_team: u32
        let arena_team = crate::util::read_u32_le(&mut r)?;

        // rating: u32
        let rating = crate::util::read_u32_le(&mut r)?;

        // games_played_this_week: u32
        let games_played_this_week = crate::util::read_u32_le(&mut r)?;

        // games_won_this_week: u32
        let games_won_this_week = crate::util::read_u32_le(&mut r)?;

        // games_played_this_season: u32
        let games_played_this_season = crate::util::read_u32_le(&mut r)?;

        // games_won_this_season: u32
        let games_won_this_season = crate::util::read_u32_le(&mut r)?;

        // ranking: u32
        let ranking = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            arena_team,
            rating,
            games_played_this_week,
            games_won_this_week,
            games_played_this_season,
            games_won_this_season,
            ranking,
        })
    }

}

impl crate::Message for SMSG_ARENA_TEAM_STATS {
    const OPCODE: u32 = 0x035b;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_ARENA_TEAM_STATS"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_ARENA_TEAM_STATS {{").unwrap();
        // Members
        writeln!(s, "    arena_team = {};", self.arena_team).unwrap();
        writeln!(s, "    rating = {};", self.rating).unwrap();
        writeln!(s, "    games_played_this_week = {};", self.games_played_this_week).unwrap();
        writeln!(s, "    games_won_this_week = {};", self.games_won_this_week).unwrap();
        writeln!(s, "    games_played_this_season = {};", self.games_played_this_season).unwrap();
        writeln!(s, "    games_won_this_season = {};", self.games_won_this_season).unwrap();
        writeln!(s, "    ranking = {};", self.ranking).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 30_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 859_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "arena_team", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "rating", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "games_played_this_week", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "games_won_this_week", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "games_played_this_season", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "games_won_this_season", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "ranking", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        28
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // arena_team: u32
        w.write_all(&self.arena_team.to_le_bytes())?;

        // rating: u32
        w.write_all(&self.rating.to_le_bytes())?;

        // games_played_this_week: u32
        w.write_all(&self.games_played_this_week.to_le_bytes())?;

        // games_won_this_week: u32
        w.write_all(&self.games_won_this_week.to_le_bytes())?;

        // games_played_this_season: u32
        w.write_all(&self.games_played_this_season.to_le_bytes())?;

        // games_won_this_season: u32
        w.write_all(&self.games_won_this_season.to_le_bytes())?;

        // ranking: u32
        w.write_all(&self.ranking.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(859, "SMSG_ARENA_TEAM_STATS", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_ARENA_TEAM_STATS {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_ARENA_TEAM_STATS {}

