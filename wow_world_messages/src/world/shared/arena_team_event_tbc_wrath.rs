/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/arena/smsg_arena_team_event.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/arena/smsg_arena_team_event.wowm#L1):
/// ```text
/// enum ArenaTeamEvent : u8 {
///     JOIN = 3;
///     LEAVE = 4;
///     REMOVE = 5;
///     LEADER_IS = 6;
///     LEADER_CHANGED = 7;
///     DISBANDED = 8;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum ArenaTeamEvent {
    /// player name + arena team name
    ///
    Join,
    /// player name + arena team name
    ///
    Leave,
    /// player name + arena team name + captain name
    ///
    Remove,
    /// player name + arena team name
    ///
    LeaderIs,
    /// old captain + new captain + arena team name
    ///
    LeaderChanged,
    /// captain name + arena team name
    ///
    Disbanded,
}

impl ArenaTeamEvent {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Join => 0x3,
            Self::Leave => 0x4,
            Self::Remove => 0x5,
            Self::LeaderIs => 0x6,
            Self::LeaderChanged => 0x7,
            Self::Disbanded => 0x8,
        }
    }

}

impl Default for ArenaTeamEvent {
    fn default() -> Self {
        Self::Join
    }
}

impl std::fmt::Display for ArenaTeamEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Join => f.write_str("Join"),
            Self::Leave => f.write_str("Leave"),
            Self::Remove => f.write_str("Remove"),
            Self::LeaderIs => f.write_str("LeaderIs"),
            Self::LeaderChanged => f.write_str("LeaderChanged"),
            Self::Disbanded => f.write_str("Disbanded"),
        }
    }
}

impl TryFrom<u8> for ArenaTeamEvent {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            3 => Ok(Self::Join),
            4 => Ok(Self::Leave),
            5 => Ok(Self::Remove),
            6 => Ok(Self::LeaderIs),
            7 => Ok(Self::LeaderChanged),
            8 => Ok(Self::Disbanded),
            v => Err(crate::errors::EnumError::new("ArenaTeamEvent", v as u64),)
        }
    }
}

