/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/social_common.wowm:66`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/social_common.wowm#L66):
/// ```text
/// enum Language : u32 {
///     UNIVERSAL = 0;
///     ORCISH = 1;
///     DARNASSIAN = 2;
///     TAURAHE = 3;
///     DWARVISH = 6;
///     COMMON = 7;
///     DEMONIC = 8;
///     TITAN = 9;
///     THALASSIAN = 10;
///     DRACONIC = 11;
///     KALIMAG = 12;
///     GNOMISH = 13;
///     TROLL = 14;
///     GUTTERSPEAK = 33;
///     ADDON = 0xFFFFFFFF;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Language {
    Universal,
    Orcish,
    Darnassian,
    Taurahe,
    Dwarvish,
    Common,
    Demonic,
    Titan,
    Thalassian,
    Draconic,
    Kalimag,
    Gnomish,
    Troll,
    Gutterspeak,
    Addon,
}

impl Language {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::Universal => 0x0,
            Self::Orcish => 0x1,
            Self::Darnassian => 0x2,
            Self::Taurahe => 0x3,
            Self::Dwarvish => 0x6,
            Self::Common => 0x7,
            Self::Demonic => 0x8,
            Self::Titan => 0x9,
            Self::Thalassian => 0xa,
            Self::Draconic => 0xb,
            Self::Kalimag => 0xc,
            Self::Gnomish => 0xd,
            Self::Troll => 0xe,
            Self::Gutterspeak => 0x21,
            Self::Addon => 0xffffffff,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl Language {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Universal => "UNIVERSAL",
            Self::Orcish => "ORCISH",
            Self::Darnassian => "DARNASSIAN",
            Self::Taurahe => "TAURAHE",
            Self::Dwarvish => "DWARVISH",
            Self::Common => "COMMON",
            Self::Demonic => "DEMONIC",
            Self::Titan => "TITAN",
            Self::Thalassian => "THALASSIAN",
            Self::Draconic => "DRACONIC",
            Self::Kalimag => "KALIMAG",
            Self::Gnomish => "GNOMISH",
            Self::Troll => "TROLL",
            Self::Gutterspeak => "GUTTERSPEAK",
            Self::Addon => "ADDON",
        }
    }

}

impl Default for Language {
    fn default() -> Self {
        Self::Universal
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Universal => f.write_str("Universal"),
            Self::Orcish => f.write_str("Orcish"),
            Self::Darnassian => f.write_str("Darnassian"),
            Self::Taurahe => f.write_str("Taurahe"),
            Self::Dwarvish => f.write_str("Dwarvish"),
            Self::Common => f.write_str("Common"),
            Self::Demonic => f.write_str("Demonic"),
            Self::Titan => f.write_str("Titan"),
            Self::Thalassian => f.write_str("Thalassian"),
            Self::Draconic => f.write_str("Draconic"),
            Self::Kalimag => f.write_str("Kalimag"),
            Self::Gnomish => f.write_str("Gnomish"),
            Self::Troll => f.write_str("Troll"),
            Self::Gutterspeak => f.write_str("Gutterspeak"),
            Self::Addon => f.write_str("Addon"),
        }
    }
}

impl TryFrom<u32> for Language {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Universal),
            1 => Ok(Self::Orcish),
            2 => Ok(Self::Darnassian),
            3 => Ok(Self::Taurahe),
            6 => Ok(Self::Dwarvish),
            7 => Ok(Self::Common),
            8 => Ok(Self::Demonic),
            9 => Ok(Self::Titan),
            10 => Ok(Self::Thalassian),
            11 => Ok(Self::Draconic),
            12 => Ok(Self::Kalimag),
            13 => Ok(Self::Gnomish),
            14 => Ok(Self::Troll),
            33 => Ok(Self::Gutterspeak),
            4294967295 => Ok(Self::Addon),
            v => Err(crate::errors::EnumError::new("Language", v as u64),)
        }
    }
}

