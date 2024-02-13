/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/smsg_update_lfg_list.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_update_lfg_list.wowm#L1):
/// ```text
/// enum LfgType : u8 {
///     NONE = 0;
///     DUNGEON = 1;
///     RAID = 2;
///     ZONE = 4;
///     HEROIC = 5;
///     RANDOM = 6;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum LfgType {
    None,
    Dungeon,
    Raid,
    Zone,
    Heroic,
    Random,
}

impl LfgType {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::None => 0x0,
            Self::Dungeon => 0x1,
            Self::Raid => 0x2,
            Self::Zone => 0x4,
            Self::Heroic => 0x5,
            Self::Random => 0x6,
        }
    }

    pub const fn variants() -> [Self; 6] {
        [
            Self::None,
            Self::Dungeon,
            Self::Raid,
            Self::Zone,
            Self::Heroic,
            Self::Random,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Dungeon),
            2 => Ok(Self::Raid),
            4 => Ok(Self::Zone),
            5 => Ok(Self::Heroic),
            6 => Ok(Self::Random),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl LfgType {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::None => "NONE",
            Self::Dungeon => "DUNGEON",
            Self::Raid => "RAID",
            Self::Zone => "ZONE",
            Self::Heroic => "HEROIC",
            Self::Random => "RANDOM",
        }
    }

}

const NAME: &str = "LfgType";

impl Default for LfgType {
    fn default() -> Self {
        Self::None
    }
}

impl std::fmt::Display for LfgType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => f.write_str("None"),
            Self::Dungeon => f.write_str("Dungeon"),
            Self::Raid => f.write_str("Raid"),
            Self::Zone => f.write_str("Zone"),
            Self::Heroic => f.write_str("Heroic"),
            Self::Random => f.write_str("Random"),
        }
    }
}

impl TryFrom<u8> for LfgType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for LfgType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for LfgType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for LfgType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for LfgType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i16> for LfgType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for LfgType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for LfgType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for LfgType {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

