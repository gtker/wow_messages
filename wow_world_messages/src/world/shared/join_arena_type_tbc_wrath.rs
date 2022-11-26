use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/cmsg_battlemaster_join_arena.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/cmsg_battlemaster_join_arena.wowm#L1):
/// ```text
/// enum JoinArenaType : u8 {
///     TWO_VS_TWO = 0;
///     THREE_VS_THREE = 1;
///     FIVE_VS_FIVE = 2;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum JoinArenaType {
    TwoVsTwo,
    ThreeVsThree,
    FiveVsFive,
}

impl JoinArenaType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::TwoVsTwo => 0x0,
            Self::ThreeVsThree => 0x1,
            Self::FiveVsFive => 0x2,
        }
    }

}

impl Default for JoinArenaType {
    fn default() -> Self {
        Self::TwoVsTwo
    }
}

impl std::fmt::Display for JoinArenaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TwoVsTwo => f.write_str("TwoVsTwo"),
            Self::ThreeVsThree => f.write_str("ThreeVsThree"),
            Self::FiveVsFive => f.write_str("FiveVsFive"),
        }
    }
}

impl TryFrom<u8> for JoinArenaType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::TwoVsTwo),
            1 => Ok(Self::ThreeVsThree),
            2 => Ok(Self::FiveVsFive),
            v => Err(crate::errors::EnumError::new("JoinArenaType", v as u32),)
        }
    }
}

