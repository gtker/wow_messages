use std::convert::{TryFrom, TryInto};

/// vmangos: From GMTicketCategory.dbc
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gamemaster/gamemaster_common.wowm:14`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/gamemaster_common.wowm#L14):
/// ```text
/// enum GmTicketType : u8 {
///     STUCK = 1;
///     BEHAVIOR_HARASSMENT = 2;
///     GUILD = 3;
///     ITEM = 4;
///     ENVIRONMENTAL = 5;
///     NONQUEST_CREEP = 6;
///     QUEST_QUESTNPC = 7;
///     TECHNICAL = 8;
///     ACCOUNT_BILLING = 9;
///     CHARACTER = 10;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum GmTicketType {
    STUCK,
    BEHAVIOR_HARASSMENT,
    GUILD,
    ITEM,
    ENVIRONMENTAL,
    NONQUEST_CREEP,
    QUEST_QUESTNPC,
    TECHNICAL,
    ACCOUNT_BILLING,
    CHARACTER,
}

impl GmTicketType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::STUCK => 0x1,
            Self::BEHAVIOR_HARASSMENT => 0x2,
            Self::GUILD => 0x3,
            Self::ITEM => 0x4,
            Self::ENVIRONMENTAL => 0x5,
            Self::NONQUEST_CREEP => 0x6,
            Self::QUEST_QUESTNPC => 0x7,
            Self::TECHNICAL => 0x8,
            Self::ACCOUNT_BILLING => 0x9,
            Self::CHARACTER => 0xa,
        }
    }

}

impl Default for GmTicketType {
    fn default() -> Self {
        Self::STUCK
    }
}

impl std::fmt::Display for GmTicketType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::STUCK => f.write_str("STUCK"),
            Self::BEHAVIOR_HARASSMENT => f.write_str("BEHAVIOR_HARASSMENT"),
            Self::GUILD => f.write_str("GUILD"),
            Self::ITEM => f.write_str("ITEM"),
            Self::ENVIRONMENTAL => f.write_str("ENVIRONMENTAL"),
            Self::NONQUEST_CREEP => f.write_str("NONQUEST_CREEP"),
            Self::QUEST_QUESTNPC => f.write_str("QUEST_QUESTNPC"),
            Self::TECHNICAL => f.write_str("TECHNICAL"),
            Self::ACCOUNT_BILLING => f.write_str("ACCOUNT_BILLING"),
            Self::CHARACTER => f.write_str("CHARACTER"),
        }
    }
}

impl TryFrom<u8> for GmTicketType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::STUCK),
            2 => Ok(Self::BEHAVIOR_HARASSMENT),
            3 => Ok(Self::GUILD),
            4 => Ok(Self::ITEM),
            5 => Ok(Self::ENVIRONMENTAL),
            6 => Ok(Self::NONQUEST_CREEP),
            7 => Ok(Self::QUEST_QUESTNPC),
            8 => Ok(Self::TECHNICAL),
            9 => Ok(Self::ACCOUNT_BILLING),
            10 => Ok(Self::CHARACTER),
            v => Err(crate::errors::EnumError::new("GmTicketType", v as u32),)
        }
    }
}

