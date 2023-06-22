/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/enums/gender.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/enums/gender.wowm#L3):
/// ```text
/// enum Gender : u8 {
///     MALE = 0;
///     FEMALE = 1;
///     NONE = 2;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Gender {
    Male,
    Female,
    /// Apparently used by hunter and warlock pets.
    None,
}

impl Gender {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Male => 0x0,
            Self::Female => 0x1,
            Self::None => 0x2,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl Gender {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Male => "MALE",
            Self::Female => "FEMALE",
            Self::None => "NONE",
        }
    }

}

impl Default for Gender {
    fn default() -> Self {
        Self::Male
    }
}

impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Male => f.write_str("Male"),
            Self::Female => f.write_str("Female"),
            Self::None => f.write_str("None"),
        }
    }
}

impl TryFrom<u8> for Gender {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Male),
            1 => Ok(Self::Female),
            2 => Ok(Self::None),
            v => Err(crate::errors::EnumError::new("Gender", v.into()),)
        }
    }
}

