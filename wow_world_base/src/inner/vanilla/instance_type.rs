/// Used in `Map.dbc`, and `LFGDungeons.dbc`.
///
/// This type is not sent over the network, but is used in the game in another way.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/external/instance_type.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/external/instance_type.wowm#L1):
/// ```text
/// enum InstanceType : u8 {
///     NORMAL = 0x00;
///     GROUP_INSTANCE = 0x01;
///     RAID_INSTANCE = 0x02;
///     BATTLEGROUND = 0x03;
///     WORLD_ZONE = 0x04;
///     BATTLEGROUND2 = 0x05;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum InstanceType {
    Normal,
    GroupInstance,
    RaidInstance,
    Battleground,
    WorldZone,
    Battleground2,
}

impl InstanceType {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Normal => 0x0,
            Self::GroupInstance => 0x1,
            Self::RaidInstance => 0x2,
            Self::Battleground => 0x3,
            Self::WorldZone => 0x4,
            Self::Battleground2 => 0x5,
        }
    }

    pub const fn variants() -> [Self; 6] {
        [
            Self::Normal,
            Self::GroupInstance,
            Self::RaidInstance,
            Self::Battleground,
            Self::WorldZone,
            Self::Battleground2,
        ]
    }

}

#[cfg(feature = "print-testcase")]
impl InstanceType {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Normal => "NORMAL",
            Self::GroupInstance => "GROUP_INSTANCE",
            Self::RaidInstance => "RAID_INSTANCE",
            Self::Battleground => "BATTLEGROUND",
            Self::WorldZone => "WORLD_ZONE",
            Self::Battleground2 => "BATTLEGROUND2",
        }
    }

}

const NAME: &str = "InstanceType";

impl Default for InstanceType {
    fn default() -> Self {
        Self::Normal
    }
}

impl std::fmt::Display for InstanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => f.write_str("Normal"),
            Self::GroupInstance => f.write_str("GroupInstance"),
            Self::RaidInstance => f.write_str("RaidInstance"),
            Self::Battleground => f.write_str("Battleground"),
            Self::WorldZone => f.write_str("WorldZone"),
            Self::Battleground2 => f.write_str("Battleground2"),
        }
    }
}

impl TryFrom<u8> for InstanceType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Normal),
            1 => Ok(Self::GroupInstance),
            2 => Ok(Self::RaidInstance),
            3 => Ok(Self::Battleground),
            4 => Ok(Self::WorldZone),
            5 => Ok(Self::Battleground2),
            v => Err(crate::errors::EnumError::new(NAME, v.into()),)
        }
    }
}

impl TryFrom<u16> for InstanceType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for InstanceType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for InstanceType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for InstanceType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for InstanceType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for InstanceType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for InstanceType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for InstanceType {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

