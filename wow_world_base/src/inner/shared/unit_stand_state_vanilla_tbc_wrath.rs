/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/social_common.wowm:453`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/social_common.wowm#L453):
/// ```text
/// enum UnitStandState : u8 {
///     STAND = 0;
///     SIT = 1;
///     SIT_CHAIR = 2;
///     SLEEP = 3;
///     SIT_LOW_CHAIR = 4;
///     SIT_MEDIUM_CHAIR = 5;
///     SIT_HIGH_CHAIR = 6;
///     DEAD = 7;
///     KNEEL = 8;
///     CUSTOM = 9;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum UnitStandState {
    Stand,
    Sit,
    SitChair,
    Sleep,
    SitLowChair,
    SitMediumChair,
    SitHighChair,
    Dead,
    Kneel,
    /// Used for Cthun according to cmangos.
    /// mangos-tbc: Depends on model animation. Submerge, freeze, hide, hibernate, rest
    Custom,
}

impl UnitStandState {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Stand => 0x0,
            Self::Sit => 0x1,
            Self::SitChair => 0x2,
            Self::Sleep => 0x3,
            Self::SitLowChair => 0x4,
            Self::SitMediumChair => 0x5,
            Self::SitHighChair => 0x6,
            Self::Dead => 0x7,
            Self::Kneel => 0x8,
            Self::Custom => 0x9,
        }
    }

    pub const fn variants() -> [Self; 10] {
        [
            Self::Stand,
            Self::Sit,
            Self::SitChair,
            Self::Sleep,
            Self::SitLowChair,
            Self::SitMediumChair,
            Self::SitHighChair,
            Self::Dead,
            Self::Kneel,
            Self::Custom,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::Stand),
            1 => Ok(Self::Sit),
            2 => Ok(Self::SitChair),
            3 => Ok(Self::Sleep),
            4 => Ok(Self::SitLowChair),
            5 => Ok(Self::SitMediumChair),
            6 => Ok(Self::SitHighChair),
            7 => Ok(Self::Dead),
            8 => Ok(Self::Kneel),
            9 => Ok(Self::Custom),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl UnitStandState {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Stand => "STAND",
            Self::Sit => "SIT",
            Self::SitChair => "SIT_CHAIR",
            Self::Sleep => "SLEEP",
            Self::SitLowChair => "SIT_LOW_CHAIR",
            Self::SitMediumChair => "SIT_MEDIUM_CHAIR",
            Self::SitHighChair => "SIT_HIGH_CHAIR",
            Self::Dead => "DEAD",
            Self::Kneel => "KNEEL",
            Self::Custom => "CUSTOM",
        }
    }

}

const NAME: &str = "UnitStandState";

impl Default for UnitStandState {
    fn default() -> Self {
        Self::Stand
    }
}

impl std::fmt::Display for UnitStandState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Stand => f.write_str("Stand"),
            Self::Sit => f.write_str("Sit"),
            Self::SitChair => f.write_str("SitChair"),
            Self::Sleep => f.write_str("Sleep"),
            Self::SitLowChair => f.write_str("SitLowChair"),
            Self::SitMediumChair => f.write_str("SitMediumChair"),
            Self::SitHighChair => f.write_str("SitHighChair"),
            Self::Dead => f.write_str("Dead"),
            Self::Kneel => f.write_str("Kneel"),
            Self::Custom => f.write_str("Custom"),
        }
    }
}

impl TryFrom<u8> for UnitStandState {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for UnitStandState {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for UnitStandState {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for UnitStandState {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for UnitStandState {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i16> for UnitStandState {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for UnitStandState {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for UnitStandState {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for UnitStandState {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

