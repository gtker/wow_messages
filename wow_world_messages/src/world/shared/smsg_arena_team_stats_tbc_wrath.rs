use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
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
impl crate::Message for SMSG_ARENA_TEAM_STATS {
    const OPCODE: u32 = 0x035b;

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
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 28 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x035B, size: body_size });
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
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_ARENA_TEAM_STATS {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_ARENA_TEAM_STATS {}

