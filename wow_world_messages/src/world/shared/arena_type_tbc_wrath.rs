use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/smsg_battlefield_status.wowm:80`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/smsg_battlefield_status.wowm#L80):
/// ```text
/// enum ArenaType : u8 {
///     NOT_ARENA = 0;
///     TWO_VS_TWO = 2;
///     THREE_VS_THREE = 3;
///     FIVE_VS_FIVE = 5;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum ArenaType {
    NotArena,
    TwoVsTwo,
    ThreeVsThree,
    FiveVsFive,
}

impl ArenaType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NotArena => 0x0,
            Self::TwoVsTwo => 0x2,
            Self::ThreeVsThree => 0x3,
            Self::FiveVsFive => 0x5,
        }
    }

}

impl Default for ArenaType {
    fn default() -> Self {
        Self::NotArena
    }
}

impl std::fmt::Display for ArenaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotArena => f.write_str("NotArena"),
            Self::TwoVsTwo => f.write_str("TwoVsTwo"),
            Self::ThreeVsThree => f.write_str("ThreeVsThree"),
            Self::FiveVsFive => f.write_str("FiveVsFive"),
        }
    }
}

impl TryFrom<u8> for ArenaType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NotArena),
            2 => Ok(Self::TwoVsTwo),
            3 => Ok(Self::ThreeVsThree),
            5 => Ok(Self::FiveVsFive),
            v => Err(crate::errors::EnumError::new("ArenaType", v as u32),)
        }
    }
}

