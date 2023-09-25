/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_feature_system_status.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_feature_system_status.wowm#L1):
/// ```text
/// enum ComplaintStatus : u8 {
///     DISABLED = 0;
///     ENABLED_WITHOUT_AUTO_IGNORE = 1;
///     ENABLED_WITH_AUTO_IGNORE = 2;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum ComplaintStatus {
    Disabled,
    EnabledWithoutAutoIgnore,
    EnabledWithAutoIgnore,
}

impl ComplaintStatus {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Disabled => 0x0,
            Self::EnabledWithoutAutoIgnore => 0x1,
            Self::EnabledWithAutoIgnore => 0x2,
        }
    }

    pub const fn variants() -> [Self; 3] {
        [
            Self::Disabled,
            Self::EnabledWithoutAutoIgnore,
            Self::EnabledWithAutoIgnore,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::Disabled),
            1 => Ok(Self::EnabledWithoutAutoIgnore),
            2 => Ok(Self::EnabledWithAutoIgnore),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl ComplaintStatus {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Disabled => "DISABLED",
            Self::EnabledWithoutAutoIgnore => "ENABLED_WITHOUT_AUTO_IGNORE",
            Self::EnabledWithAutoIgnore => "ENABLED_WITH_AUTO_IGNORE",
        }
    }

}

const NAME: &str = "ComplaintStatus";

impl Default for ComplaintStatus {
    fn default() -> Self {
        Self::Disabled
    }
}

impl std::fmt::Display for ComplaintStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Disabled => f.write_str("Disabled"),
            Self::EnabledWithoutAutoIgnore => f.write_str("EnabledWithoutAutoIgnore"),
            Self::EnabledWithAutoIgnore => f.write_str("EnabledWithAutoIgnore"),
        }
    }
}

impl TryFrom<u8> for ComplaintStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for ComplaintStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for ComplaintStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for ComplaintStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for ComplaintStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for ComplaintStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for ComplaintStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for ComplaintStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for ComplaintStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

