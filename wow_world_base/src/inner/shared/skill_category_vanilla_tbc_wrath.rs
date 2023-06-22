/// This type is not sent over the network, but is used in the game in another way.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/external/skill_category.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/external/skill_category.wowm#L3):
/// ```text
/// enum SkillCategory : u8 {
///     ATTRIBUTE = 5;
///     WEAPON = 6;
///     CLASS = 7;
///     ARMOR = 8;
///     SECONDARY_PROFESSION = 9;
///     LANGUAGE = 10;
///     PRIMARY_PROFESSION = 11;
///     GENERIC = 12;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum SkillCategory {
    /// Not used for anything in Vanilla and TBC, only used for `Pet - Exotic Spirit Beast` in Wrath.
    Attribute,
    Weapon,
    Class,
    Armor,
    SecondaryProfession,
    Language,
    PrimaryProfession,
    Generic,
}

impl SkillCategory {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Attribute => 0x5,
            Self::Weapon => 0x6,
            Self::Class => 0x7,
            Self::Armor => 0x8,
            Self::SecondaryProfession => 0x9,
            Self::Language => 0xa,
            Self::PrimaryProfession => 0xb,
            Self::Generic => 0xc,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl SkillCategory {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Attribute => "ATTRIBUTE",
            Self::Weapon => "WEAPON",
            Self::Class => "CLASS",
            Self::Armor => "ARMOR",
            Self::SecondaryProfession => "SECONDARY_PROFESSION",
            Self::Language => "LANGUAGE",
            Self::PrimaryProfession => "PRIMARY_PROFESSION",
            Self::Generic => "GENERIC",
        }
    }

}

const NAME: &str = "SkillCategory";

impl Default for SkillCategory {
    fn default() -> Self {
        Self::Attribute
    }
}

impl std::fmt::Display for SkillCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Attribute => f.write_str("Attribute"),
            Self::Weapon => f.write_str("Weapon"),
            Self::Class => f.write_str("Class"),
            Self::Armor => f.write_str("Armor"),
            Self::SecondaryProfession => f.write_str("SecondaryProfession"),
            Self::Language => f.write_str("Language"),
            Self::PrimaryProfession => f.write_str("PrimaryProfession"),
            Self::Generic => f.write_str("Generic"),
        }
    }
}

impl TryFrom<u8> for SkillCategory {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            5 => Ok(Self::Attribute),
            6 => Ok(Self::Weapon),
            7 => Ok(Self::Class),
            8 => Ok(Self::Armor),
            9 => Ok(Self::SecondaryProfession),
            10 => Ok(Self::Language),
            11 => Ok(Self::PrimaryProfession),
            12 => Ok(Self::Generic),
            v => Err(crate::errors::EnumError::new(NAME, v.into()),)
        }
    }
}

impl TryFrom<u16> for SkillCategory {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for SkillCategory {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for SkillCategory {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for SkillCategory {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for SkillCategory {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for SkillCategory {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for SkillCategory {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for SkillCategory {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

