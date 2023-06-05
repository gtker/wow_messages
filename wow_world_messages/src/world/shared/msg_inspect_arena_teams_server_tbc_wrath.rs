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

impl crate::private::Sealed for MSG_INSPECT_ARENA_TEAMS_Server {}
impl crate::Message for MSG_INSPECT_ARENA_TEAMS_Server {
    const OPCODE: u32 = 0x0377;

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

