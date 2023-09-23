/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_group_list.wowm:79`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_group_list.wowm#L79):
/// ```text
/// enum RaidDifficulty : u8 {
///     TEN_MAN_NORMAL = 0;
///     TWENTY_FIVE_MAN_NORMAL = 1;
///     TEN_MAN_HEROIC = 2;
///     TWENTY_FIVE_MAN_HEROIC = 3;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum RaidDifficulty {
    TenManNormal,
    TwentyFiveManNormal,
    TenManHeroic,
    TwentyFiveManHeroic,
}

impl RaidDifficulty {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::TenManNormal => 0x0,
            Self::TwentyFiveManNormal => 0x1,
            Self::TenManHeroic => 0x2,
            Self::TwentyFiveManHeroic => 0x3,
        }
    }

    pub const fn variants() -> [Self; 4] {
        [
            Self::TenManNormal,
            Self::TwentyFiveManNormal,
            Self::TenManHeroic,
            Self::TwentyFiveManHeroic,
        ]
    }

}

#[cfg(feature = "print-testcase")]
impl RaidDifficulty {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::TenManNormal => "TEN_MAN_NORMAL",
            Self::TwentyFiveManNormal => "TWENTY_FIVE_MAN_NORMAL",
            Self::TenManHeroic => "TEN_MAN_HEROIC",
            Self::TwentyFiveManHeroic => "TWENTY_FIVE_MAN_HEROIC",
        }
    }

}

const NAME: &str = "RaidDifficulty";

impl Default for RaidDifficulty {
    fn default() -> Self {
        Self::TenManNormal
    }
}

impl std::fmt::Display for RaidDifficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TenManNormal => f.write_str("TenManNormal"),
            Self::TwentyFiveManNormal => f.write_str("TwentyFiveManNormal"),
            Self::TenManHeroic => f.write_str("TenManHeroic"),
            Self::TwentyFiveManHeroic => f.write_str("TwentyFiveManHeroic"),
        }
    }
}

impl TryFrom<u8> for RaidDifficulty {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::TenManNormal),
            1 => Ok(Self::TwentyFiveManNormal),
            2 => Ok(Self::TenManHeroic),
            3 => Ok(Self::TwentyFiveManHeroic),
            v => Err(crate::errors::EnumError::new(NAME, v.into()),)
        }
    }
}

impl TryFrom<u16> for RaidDifficulty {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for RaidDifficulty {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for RaidDifficulty {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for RaidDifficulty {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for RaidDifficulty {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for RaidDifficulty {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for RaidDifficulty {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for RaidDifficulty {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

