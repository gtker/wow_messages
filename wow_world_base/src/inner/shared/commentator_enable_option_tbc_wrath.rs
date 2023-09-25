/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/cmsg_commentator_enable.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_commentator_enable.wowm#L1):
/// ```text
/// enum CommentatorEnableOption : u32 {
///     DISABLE = 0;
///     ENABLE = 1;
///     TOGGLE = 2;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum CommentatorEnableOption {
    Disable,
    Enable,
    Toggle,
}

impl CommentatorEnableOption {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::Disable => 0x0,
            Self::Enable => 0x1,
            Self::Toggle => 0x2,
        }
    }

    pub const fn variants() -> [Self; 3] {
        [
            Self::Disable,
            Self::Enable,
            Self::Toggle,
        ]
    }

    pub const fn from_int(value: u32) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::Disable),
            1 => Ok(Self::Enable),
            2 => Ok(Self::Toggle),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl CommentatorEnableOption {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Disable => "DISABLE",
            Self::Enable => "ENABLE",
            Self::Toggle => "TOGGLE",
        }
    }

}

const NAME: &str = "CommentatorEnableOption";

impl Default for CommentatorEnableOption {
    fn default() -> Self {
        Self::Disable
    }
}

impl std::fmt::Display for CommentatorEnableOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Disable => f.write_str("Disable"),
            Self::Enable => f.write_str("Enable"),
            Self::Toggle => f.write_str("Toggle"),
        }
    }
}

impl TryFrom<u32> for CommentatorEnableOption {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u8> for CommentatorEnableOption {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u16> for CommentatorEnableOption {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for CommentatorEnableOption {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for CommentatorEnableOption {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for CommentatorEnableOption {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for CommentatorEnableOption {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for CommentatorEnableOption {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for CommentatorEnableOption {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

