/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/msg_looking_for_group.wowm:22`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/msg_looking_for_group.wowm#L22):
/// ```text
/// enum LfgMode : u8 {
///     LOOKING_FOR_GROUP = 0;
///     LOOKING_FOR_MORE = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum LfgMode {
    LookingForGroup,
    LookingForMore,
}

impl LfgMode {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::LookingForGroup => 0x0,
            Self::LookingForMore => 0x1,
        }
    }

    pub const fn variants() -> [Self; 2] {
        [
            Self::LookingForGroup,
            Self::LookingForMore,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::LookingForGroup),
            1 => Ok(Self::LookingForMore),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl LfgMode {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::LookingForGroup => "LOOKING_FOR_GROUP",
            Self::LookingForMore => "LOOKING_FOR_MORE",
        }
    }

}

const NAME: &str = "LfgMode";

impl Default for LfgMode {
    fn default() -> Self {
        Self::LookingForGroup
    }
}

impl std::fmt::Display for LfgMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LookingForGroup => f.write_str("LookingForGroup"),
            Self::LookingForMore => f.write_str("LookingForMore"),
        }
    }
}

impl TryFrom<u8> for LfgMode {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for LfgMode {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for LfgMode {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for LfgMode {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for LfgMode {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for LfgMode {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for LfgMode {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for LfgMode {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for LfgMode {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

