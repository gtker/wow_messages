/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/cmsg_battlemaster_join_arena.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/cmsg_battlemaster_join_arena.wowm#L1):
/// ```text
/// enum JoinArenaType : u8 {
///     TWO_VS_TWO = 0;
///     THREE_VS_THREE = 1;
///     FIVE_VS_FIVE = 2;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum JoinArenaType {
    TwoVsTwo,
    ThreeVsThree,
    FiveVsFive,
}

impl JoinArenaType {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::TwoVsTwo => 0x0,
            Self::ThreeVsThree => 0x1,
            Self::FiveVsFive => 0x2,
        }
    }

    pub const fn variants() -> [Self; 3] {
        [
            Self::TwoVsTwo,
            Self::ThreeVsThree,
            Self::FiveVsFive,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::TwoVsTwo),
            1 => Ok(Self::ThreeVsThree),
            2 => Ok(Self::FiveVsFive),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl JoinArenaType {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::TwoVsTwo => "TWO_VS_TWO",
            Self::ThreeVsThree => "THREE_VS_THREE",
            Self::FiveVsFive => "FIVE_VS_FIVE",
        }
    }

}

const NAME: &str = "JoinArenaType";

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
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for JoinArenaType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for JoinArenaType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for JoinArenaType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for JoinArenaType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i16> for JoinArenaType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for JoinArenaType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for JoinArenaType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for JoinArenaType {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

