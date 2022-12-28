use crate::EnumError;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
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
            SkillCategory::Attribute => 5,
            SkillCategory::Weapon => 6,
            SkillCategory::Class => 7,
            SkillCategory::Armor => 8,
            SkillCategory::SecondaryProfession => 9,
            SkillCategory::Language => 10,
            SkillCategory::PrimaryProfession => 11,
            SkillCategory::Generic => 12,
        }
    }
}

impl Display for SkillCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            SkillCategory::Attribute => "Attribute",
            SkillCategory::Weapon => "Weapon",
            SkillCategory::Class => "Class",
            SkillCategory::Armor => "Armor",
            SkillCategory::SecondaryProfession => "Secondary Profession",
            SkillCategory::Language => "Language",
            SkillCategory::PrimaryProfession => "Primary Profession",
            SkillCategory::Generic => "Generic",
        })
    }
}

impl TryFrom<u8> for SkillCategory {
    type Error = EnumError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            5 => SkillCategory::Attribute,
            6 => SkillCategory::Weapon,
            7 => SkillCategory::Class,
            8 => SkillCategory::Armor,
            9 => SkillCategory::SecondaryProfession,
            10 => SkillCategory::Language,
            11 => SkillCategory::PrimaryProfession,
            12 => SkillCategory::Generic,
            v => return Err(EnumError::new("SkillCategory", v.into())),
        })
    }
}
