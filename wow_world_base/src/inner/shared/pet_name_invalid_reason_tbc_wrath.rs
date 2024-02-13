/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/smsg_pet_name_invalid.wowm:6`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/smsg_pet_name_invalid.wowm#L6):
/// ```text
/// enum PetNameInvalidReason : u8 {
///     INVALID = 1;
///     NO_NAME = 2;
///     TOO_SHORT = 3;
///     TOO_LONG = 4;
///     MIXED_LANGUAGES = 6;
///     PROFANE = 7;
///     RESERVED = 8;
///     THREE_CONSECUTIVE = 11;
///     INVALID_SPACE = 12;
///     CONSECUTIVE_SPACES = 13;
///     RUSSIAN_CONSECUTIVE_SILENT_CHARACTERS = 14;
///     RUSSIAN_SILENT_CHARACTER_AT_BEGINNING_OR_END = 15;
///     DECLENSION_DOESNT_MATCH_BASE_NAME = 16;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum PetNameInvalidReason {
    Invalid,
    NoName,
    TooShort,
    TooLong,
    MixedLanguages,
    Profane,
    Reserved,
    ThreeConsecutive,
    InvalidSpace,
    ConsecutiveSpaces,
    RussianConsecutiveSilentCharacters,
    RussianSilentCharacterAtBeginningOrEnd,
    DeclensionDoesntMatchBaseName,
}

impl PetNameInvalidReason {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Invalid => 0x1,
            Self::NoName => 0x2,
            Self::TooShort => 0x3,
            Self::TooLong => 0x4,
            Self::MixedLanguages => 0x6,
            Self::Profane => 0x7,
            Self::Reserved => 0x8,
            Self::ThreeConsecutive => 0xb,
            Self::InvalidSpace => 0xc,
            Self::ConsecutiveSpaces => 0xd,
            Self::RussianConsecutiveSilentCharacters => 0xe,
            Self::RussianSilentCharacterAtBeginningOrEnd => 0xf,
            Self::DeclensionDoesntMatchBaseName => 0x10,
        }
    }

    pub const fn variants() -> [Self; 13] {
        [
            Self::Invalid,
            Self::NoName,
            Self::TooShort,
            Self::TooLong,
            Self::MixedLanguages,
            Self::Profane,
            Self::Reserved,
            Self::ThreeConsecutive,
            Self::InvalidSpace,
            Self::ConsecutiveSpaces,
            Self::RussianConsecutiveSilentCharacters,
            Self::RussianSilentCharacterAtBeginningOrEnd,
            Self::DeclensionDoesntMatchBaseName,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
        match value {
            1 => Ok(Self::Invalid),
            2 => Ok(Self::NoName),
            3 => Ok(Self::TooShort),
            4 => Ok(Self::TooLong),
            6 => Ok(Self::MixedLanguages),
            7 => Ok(Self::Profane),
            8 => Ok(Self::Reserved),
            11 => Ok(Self::ThreeConsecutive),
            12 => Ok(Self::InvalidSpace),
            13 => Ok(Self::ConsecutiveSpaces),
            14 => Ok(Self::RussianConsecutiveSilentCharacters),
            15 => Ok(Self::RussianSilentCharacterAtBeginningOrEnd),
            16 => Ok(Self::DeclensionDoesntMatchBaseName),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl PetNameInvalidReason {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Invalid => "INVALID",
            Self::NoName => "NO_NAME",
            Self::TooShort => "TOO_SHORT",
            Self::TooLong => "TOO_LONG",
            Self::MixedLanguages => "MIXED_LANGUAGES",
            Self::Profane => "PROFANE",
            Self::Reserved => "RESERVED",
            Self::ThreeConsecutive => "THREE_CONSECUTIVE",
            Self::InvalidSpace => "INVALID_SPACE",
            Self::ConsecutiveSpaces => "CONSECUTIVE_SPACES",
            Self::RussianConsecutiveSilentCharacters => "RUSSIAN_CONSECUTIVE_SILENT_CHARACTERS",
            Self::RussianSilentCharacterAtBeginningOrEnd => "RUSSIAN_SILENT_CHARACTER_AT_BEGINNING_OR_END",
            Self::DeclensionDoesntMatchBaseName => "DECLENSION_DOESNT_MATCH_BASE_NAME",
        }
    }

}

const NAME: &str = "PetNameInvalidReason";

impl Default for PetNameInvalidReason {
    fn default() -> Self {
        Self::Invalid
    }
}

impl std::fmt::Display for PetNameInvalidReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid => f.write_str("Invalid"),
            Self::NoName => f.write_str("NoName"),
            Self::TooShort => f.write_str("TooShort"),
            Self::TooLong => f.write_str("TooLong"),
            Self::MixedLanguages => f.write_str("MixedLanguages"),
            Self::Profane => f.write_str("Profane"),
            Self::Reserved => f.write_str("Reserved"),
            Self::ThreeConsecutive => f.write_str("ThreeConsecutive"),
            Self::InvalidSpace => f.write_str("InvalidSpace"),
            Self::ConsecutiveSpaces => f.write_str("ConsecutiveSpaces"),
            Self::RussianConsecutiveSilentCharacters => f.write_str("RussianConsecutiveSilentCharacters"),
            Self::RussianSilentCharacterAtBeginningOrEnd => f.write_str("RussianSilentCharacterAtBeginningOrEnd"),
            Self::DeclensionDoesntMatchBaseName => f.write_str("DeclensionDoesntMatchBaseName"),
        }
    }
}

impl TryFrom<u8> for PetNameInvalidReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for PetNameInvalidReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for PetNameInvalidReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for PetNameInvalidReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for PetNameInvalidReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i16> for PetNameInvalidReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for PetNameInvalidReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for PetNameInvalidReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for PetNameInvalidReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

