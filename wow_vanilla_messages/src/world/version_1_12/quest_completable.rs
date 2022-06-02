use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_questgiver_request_item.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_questgiver_request_item.wowm#L9):
/// ```text
/// enum QuestCompletable : u32 {
///     NOT_COMPLETABLE = 0;
///     COMPLETEABLE = 3;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum QuestCompletable {
    NOT_COMPLETABLE,
    COMPLETEABLE,
}

impl QuestCompletable {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::NOT_COMPLETABLE => 0x0,
            Self::COMPLETEABLE => 0x3,
        }
    }

}

impl Default for QuestCompletable {
    fn default() -> Self {
        Self::NOT_COMPLETABLE
    }
}

impl std::fmt::Display for QuestCompletable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NOT_COMPLETABLE => f.write_str("NOT_COMPLETABLE"),
            Self::COMPLETEABLE => f.write_str("COMPLETEABLE"),
        }
    }
}

impl TryFrom<u32> for QuestCompletable {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NOT_COMPLETABLE),
            3 => Ok(Self::COMPLETEABLE),
            v => Err(crate::errors::EnumError::new("QuestCompletable", v as u32),)
        }
    }
}

