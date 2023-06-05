/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_group_list.wowm:86`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_group_list.wowm#L86):
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

}

#[cfg(feature = "print-testcase")]
impl RaidDifficulty {
    pub fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::TenManNormal => "TEN_MAN_NORMAL",
            Self::TwentyFiveManNormal => "TWENTY_FIVE_MAN_NORMAL",
            Self::TenManHeroic => "TEN_MAN_HEROIC",
            Self::TwentyFiveManHeroic => "TWENTY_FIVE_MAN_HEROIC",
        }
    }

}

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
            v => Err(crate::errors::EnumError::new("RaidDifficulty", v as u64),)
        }
    }
}

