use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::Class;
use wow_world_base::shared::level_vanilla_tbc_wrath::Level;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/arena/smsg_arena_team_roster.wowm:17`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/arena/smsg_arena_team_roster.wowm#L17):
/// ```text
/// struct ArenaTeamMember {
///     Guid guid;
///     Bool online;
///     CString name;
///     Level level;
///     Class class;
///     u32 games_played_this_week;
///     u32 wins_this_week;
///     u32 games_played_this_season;
///     u32 wins_this_season;
///     u32 personal_rating;
/// }
/// ```
pub struct ArenaTeamMember {
    pub guid: Guid,
    pub online: bool,
    pub name: String,
    pub level: Level,
    pub class: Class,
    pub games_played_this_week: u32,
    pub wins_this_week: u32,
    pub games_played_this_season: u32,
    pub wins_this_season: u32,
    pub personal_rating: u32,
}

impl ArenaTeamMember {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // online: Bool
        w.write_all(u8::from(self.online).to_le_bytes().as_slice())?;

        // name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.name.as_bytes().iter().rev().next(), Some(&0_u8), "String `name` must not be null-terminated.");
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // level: Level
        w.write_all(&self.level.as_int().to_le_bytes())?;

        // class: Class
        w.write_all(&(self.class.as_int().to_le_bytes()))?;

        // games_played_this_week: u32
        w.write_all(&self.games_played_this_week.to_le_bytes())?;

        // wins_this_week: u32
        w.write_all(&self.wins_this_week.to_le_bytes())?;

        // games_played_this_season: u32
        w.write_all(&self.games_played_this_season.to_le_bytes())?;

        // wins_this_season: u32
        w.write_all(&self.wins_this_season.to_le_bytes())?;

        // personal_rating: u32
        w.write_all(&self.personal_rating.to_le_bytes())?;

        Ok(())
    }
}

impl ArenaTeamMember {
    pub(crate) fn read<R: Read>(mut r: R) -> Result<Self, crate::errors::ParseError> {
        // guid: Guid
        let guid = Guid::read(&mut r)?;

        // online: Bool
        let online = crate::util::read_u8_le(&mut r)? != 0;

        // name: CString
        let name = {
            let name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(name)?
        };

        // level: Level
        let level = Level::new(crate::util::read_u8_le(&mut r)?);

        // class: Class
        let class: Class = crate::util::read_u8_le(&mut r)?.try_into()?;

        // games_played_this_week: u32
        let games_played_this_week = crate::util::read_u32_le(&mut r)?;

        // wins_this_week: u32
        let wins_this_week = crate::util::read_u32_le(&mut r)?;

        // games_played_this_season: u32
        let games_played_this_season = crate::util::read_u32_le(&mut r)?;

        // wins_this_season: u32
        let wins_this_season = crate::util::read_u32_le(&mut r)?;

        // personal_rating: u32
        let personal_rating = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            guid,
            online,
            name,
            level,
            class,
            games_played_this_week,
            wins_this_week,
            games_played_this_season,
            wins_this_season,
            personal_rating,
        })
    }

}

impl ArenaTeamMember {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + 1 // online: Bool
        + self.name.len() + 1 // name: CString
        + 1 // level: Level
        + 1 // class: Class
        + 4 // games_played_this_week: u32
        + 4 // wins_this_week: u32
        + 4 // games_played_this_season: u32
        + 4 // wins_this_season: u32
        + 4 // personal_rating: u32
    }
}

