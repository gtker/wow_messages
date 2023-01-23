use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/smsg_lfg_update_player.wowm:20`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_lfg_update_player.wowm#L20):
/// ```text
/// enum LfgJoinStatus : u8 {
///     NOT_JOINED = 0;
///     JOINED = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub(crate) enum LfgJoinStatus {
    NotJoined,
    Joined,
}

impl LfgJoinStatus {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NotJoined => 0x0,
            Self::Joined => 0x1,
        }
    }

}

impl Default for LfgJoinStatus {
    fn default() -> Self {
        Self::NotJoined
    }
}

impl std::fmt::Display for LfgJoinStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotJoined => f.write_str("NotJoined"),
            Self::Joined => f.write_str("Joined"),
        }
    }
}

impl TryFrom<u8> for LfgJoinStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NotJoined),
            1 => Ok(Self::Joined),
            v => Err(crate::errors::EnumError::new("LfgJoinStatus", v as u64),)
        }
    }
}

