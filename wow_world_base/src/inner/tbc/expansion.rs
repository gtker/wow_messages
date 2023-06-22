/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_auth_response.wowm:59`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_auth_response.wowm#L59):
/// ```text
/// enum Expansion : u8 {
///     VANILLA = 0;
///     THE_BURNING_CRUSADE = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Expansion {
    Vanilla,
    TheBurningCrusade,
}

impl Expansion {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Vanilla => 0x0,
            Self::TheBurningCrusade => 0x1,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl Expansion {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Vanilla => "VANILLA",
            Self::TheBurningCrusade => "THE_BURNING_CRUSADE",
        }
    }

}

impl Default for Expansion {
    fn default() -> Self {
        Self::Vanilla
    }
}

impl std::fmt::Display for Expansion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Vanilla => f.write_str("Vanilla"),
            Self::TheBurningCrusade => f.write_str("TheBurningCrusade"),
        }
    }
}

impl TryFrom<u8> for Expansion {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Vanilla),
            1 => Ok(Self::TheBurningCrusade),
            v => Err(crate::errors::EnumError::new("Expansion", v.into()),)
        }
    }
}

