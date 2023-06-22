/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/raid_target.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/raid_target.wowm#L8):
/// ```text
/// enum RaidTargetIndex : u8 {
///     UNKNOWN0 = 0;
///     UNKNOWN1 = 1;
///     UNKNOWN2 = 2;
///     UNKNOWN3 = 3;
///     UNKNOWN4 = 4;
///     UNKNOWN5 = 5;
///     UNKNOWN6 = 6;
///     UNKNOWN7 = 7;
///     UNKNOWN8 = 8;
///     REQUEST_ICONS = 0xFF;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum RaidTargetIndex {
    Unknown0,
    Unknown1,
    Unknown2,
    Unknown3,
    Unknown4,
    Unknown5,
    Unknown6,
    Unknown7,
    Unknown8,
    RequestIcons,
}

impl RaidTargetIndex {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Unknown0 => 0x0,
            Self::Unknown1 => 0x1,
            Self::Unknown2 => 0x2,
            Self::Unknown3 => 0x3,
            Self::Unknown4 => 0x4,
            Self::Unknown5 => 0x5,
            Self::Unknown6 => 0x6,
            Self::Unknown7 => 0x7,
            Self::Unknown8 => 0x8,
            Self::RequestIcons => 0xff,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl RaidTargetIndex {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Unknown0 => "UNKNOWN0",
            Self::Unknown1 => "UNKNOWN1",
            Self::Unknown2 => "UNKNOWN2",
            Self::Unknown3 => "UNKNOWN3",
            Self::Unknown4 => "UNKNOWN4",
            Self::Unknown5 => "UNKNOWN5",
            Self::Unknown6 => "UNKNOWN6",
            Self::Unknown7 => "UNKNOWN7",
            Self::Unknown8 => "UNKNOWN8",
            Self::RequestIcons => "REQUEST_ICONS",
        }
    }

}

const NAME: &str = "RaidTargetIndex";

impl Default for RaidTargetIndex {
    fn default() -> Self {
        Self::Unknown0
    }
}

impl std::fmt::Display for RaidTargetIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown0 => f.write_str("Unknown0"),
            Self::Unknown1 => f.write_str("Unknown1"),
            Self::Unknown2 => f.write_str("Unknown2"),
            Self::Unknown3 => f.write_str("Unknown3"),
            Self::Unknown4 => f.write_str("Unknown4"),
            Self::Unknown5 => f.write_str("Unknown5"),
            Self::Unknown6 => f.write_str("Unknown6"),
            Self::Unknown7 => f.write_str("Unknown7"),
            Self::Unknown8 => f.write_str("Unknown8"),
            Self::RequestIcons => f.write_str("RequestIcons"),
        }
    }
}

impl TryFrom<u8> for RaidTargetIndex {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Unknown0),
            1 => Ok(Self::Unknown1),
            2 => Ok(Self::Unknown2),
            3 => Ok(Self::Unknown3),
            4 => Ok(Self::Unknown4),
            5 => Ok(Self::Unknown5),
            6 => Ok(Self::Unknown6),
            7 => Ok(Self::Unknown7),
            8 => Ok(Self::Unknown8),
            255 => Ok(Self::RequestIcons),
            v => Err(crate::errors::EnumError::new(NAME, v.into()),)
        }
    }
}

impl TryFrom<u16> for RaidTargetIndex {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for RaidTargetIndex {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for RaidTargetIndex {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for RaidTargetIndex {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for RaidTargetIndex {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for RaidTargetIndex {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for RaidTargetIndex {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for RaidTargetIndex {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

