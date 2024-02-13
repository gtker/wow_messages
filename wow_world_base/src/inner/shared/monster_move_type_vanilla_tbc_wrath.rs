/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm#L1):
/// ```text
/// enum MonsterMoveType : u8 {
///     NORMAL = 0;
///     STOP = 1;
///     FACING_SPOT = 2;
///     FACING_TARGET = 3;
///     FACING_ANGLE = 4;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum MonsterMoveType {
    Normal,
    Stop,
    FacingSpot,
    FacingTarget,
    FacingAngle,
}

impl MonsterMoveType {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Normal => 0x0,
            Self::Stop => 0x1,
            Self::FacingSpot => 0x2,
            Self::FacingTarget => 0x3,
            Self::FacingAngle => 0x4,
        }
    }

    pub const fn variants() -> [Self; 5] {
        [
            Self::Normal,
            Self::Stop,
            Self::FacingSpot,
            Self::FacingTarget,
            Self::FacingAngle,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::Normal),
            1 => Ok(Self::Stop),
            2 => Ok(Self::FacingSpot),
            3 => Ok(Self::FacingTarget),
            4 => Ok(Self::FacingAngle),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl MonsterMoveType {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Normal => "NORMAL",
            Self::Stop => "STOP",
            Self::FacingSpot => "FACING_SPOT",
            Self::FacingTarget => "FACING_TARGET",
            Self::FacingAngle => "FACING_ANGLE",
        }
    }

}

const NAME: &str = "MonsterMoveType";

impl Default for MonsterMoveType {
    fn default() -> Self {
        Self::Normal
    }
}

impl std::fmt::Display for MonsterMoveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => f.write_str("Normal"),
            Self::Stop => f.write_str("Stop"),
            Self::FacingSpot => f.write_str("FacingSpot"),
            Self::FacingTarget => f.write_str("FacingTarget"),
            Self::FacingAngle => f.write_str("FacingAngle"),
        }
    }
}

impl TryFrom<u8> for MonsterMoveType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for MonsterMoveType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for MonsterMoveType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for MonsterMoveType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for MonsterMoveType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i16> for MonsterMoveType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for MonsterMoveType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for MonsterMoveType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for MonsterMoveType {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

