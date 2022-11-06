use crate::vanilla::Gender;
use std::convert::TryFrom;

pub mod tbc;
pub mod vanilla;
pub mod wrath;

/// Enum containing only Male and Female.
///
/// Player characters must be either male or female for e.g. display ids
/// and can not legally choose anything else through the client.
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum PlayerGender {
    Male,
    Female,
}

impl TryFrom<Gender> for PlayerGender {
    type Error = Gender;

    fn try_from(value: Gender) -> Result<Self, Self::Error> {
        Ok(match value {
            Gender::Male => Self::Male,
            Gender::Female => Self::Female,
            gender => return Err(gender),
        })
    }
}

impl Default for PlayerGender {
    fn default() -> Self {
        Self::Male
    }
}

impl std::fmt::Display for PlayerGender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            PlayerGender::Male => "Male",
            PlayerGender::Female => "Female",
        })
    }
}
