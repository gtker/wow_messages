/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/smsg_battlefield_status.wowm:71`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/smsg_battlefield_status.wowm#L71):
/// ```text
/// enum ArenaType : u8 {
///     NOT_ARENA = 0;
///     TWO_VS_TWO = 2;
///     THREE_VS_THREE = 3;
///     FIVE_VS_FIVE = 5;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum ArenaType {
    NotArena,
    TwoVsTwo,
    ThreeVsThree,
    FiveVsFive,
}

impl ArenaType {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::NotArena => 0x0,
            Self::TwoVsTwo => 0x2,
            Self::ThreeVsThree => 0x3,
            Self::FiveVsFive => 0x5,
        }
    }

    pub const fn variants() -> [Self; 4] {
        [
            Self::NotArena,
            Self::TwoVsTwo,
            Self::ThreeVsThree,
            Self::FiveVsFive,
        ]
    }

}

#[cfg(feature = "print-testcase")]
impl ArenaType {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::NotArena => "NOT_ARENA",
            Self::TwoVsTwo => "TWO_VS_TWO",
            Self::ThreeVsThree => "THREE_VS_THREE",
            Self::FiveVsFive => "FIVE_VS_FIVE",
        }
    }

}

const NAME: &str = "ArenaType";

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
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NotArena),
            2 => Ok(Self::TwoVsTwo),
            3 => Ok(Self::ThreeVsThree),
            5 => Ok(Self::FiveVsFive),
            v => Err(crate::errors::EnumError::new(NAME, v.into()),)
        }
    }
}

impl TryFrom<u16> for ArenaType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for ArenaType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for ArenaType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for ArenaType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for ArenaType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for ArenaType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for ArenaType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for ArenaType {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

