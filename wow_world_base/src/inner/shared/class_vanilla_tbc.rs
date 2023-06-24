/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/enums/class.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/enums/class.wowm#L1):
/// ```text
/// enum Class : u8 {
///     WARRIOR = 1;
///     PALADIN = 2;
///     HUNTER = 3;
///     ROGUE = 4;
///     PRIEST = 5;
///     SHAMAN = 7;
///     MAGE = 8;
///     WARLOCK = 9;
///     DRUID = 11;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Class {
    Warrior,
    Paladin,
    Hunter,
    Rogue,
    Priest,
    Shaman,
    Mage,
    Warlock,
    Druid,
}

impl Class {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Warrior => 0x1,
            Self::Paladin => 0x2,
            Self::Hunter => 0x3,
            Self::Rogue => 0x4,
            Self::Priest => 0x5,
            Self::Shaman => 0x7,
            Self::Mage => 0x8,
            Self::Warlock => 0x9,
            Self::Druid => 0xb,
        }
    }

    pub const fn variants() -> [Self; 9] {
        [
            Self::Warrior,
            Self::Paladin,
            Self::Hunter,
            Self::Rogue,
            Self::Priest,
            Self::Shaman,
            Self::Mage,
            Self::Warlock,
            Self::Druid,
        ]
    }

}

#[cfg(feature = "print-testcase")]
impl Class {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Warrior => "WARRIOR",
            Self::Paladin => "PALADIN",
            Self::Hunter => "HUNTER",
            Self::Rogue => "ROGUE",
            Self::Priest => "PRIEST",
            Self::Shaman => "SHAMAN",
            Self::Mage => "MAGE",
            Self::Warlock => "WARLOCK",
            Self::Druid => "DRUID",
        }
    }

}

const NAME: &str = "Class";

impl Default for Class {
    fn default() -> Self {
        Self::Warrior
    }
}

impl std::fmt::Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Warrior => f.write_str("Warrior"),
            Self::Paladin => f.write_str("Paladin"),
            Self::Hunter => f.write_str("Hunter"),
            Self::Rogue => f.write_str("Rogue"),
            Self::Priest => f.write_str("Priest"),
            Self::Shaman => f.write_str("Shaman"),
            Self::Mage => f.write_str("Mage"),
            Self::Warlock => f.write_str("Warlock"),
            Self::Druid => f.write_str("Druid"),
        }
    }
}

impl TryFrom<u8> for Class {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Warrior),
            2 => Ok(Self::Paladin),
            3 => Ok(Self::Hunter),
            4 => Ok(Self::Rogue),
            5 => Ok(Self::Priest),
            7 => Ok(Self::Shaman),
            8 => Ok(Self::Mage),
            9 => Ok(Self::Warlock),
            11 => Ok(Self::Druid),
            v => Err(crate::errors::EnumError::new(NAME, v.into()),)
        }
    }
}

impl TryFrom<u16> for Class {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for Class {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for Class {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for Class {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for Class {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for Class {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for Class {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for Class {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

