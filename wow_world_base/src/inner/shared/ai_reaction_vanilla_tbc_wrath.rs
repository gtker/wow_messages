/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_ai_reaction.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_ai_reaction.wowm#L3):
/// ```text
/// enum AiReaction : u32 {
///     ALERT = 0;
///     FRIENDLY = 1;
///     HOSTILE = 2;
///     AFRAID = 3;
///     DESTROY = 4;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum AiReaction {
    /// pre-aggro (used in client packet handler)
    Alert,
    /// (NOT used in client packet handler)
    Friendly,
    /// sent on every attack, triggers aggro sound (used in client packet handler)
    Hostile,
    /// seen for polymorph (when AI not in control of self?) (NOT used in client packet handler)
    Afraid,
    /// used on object destroy (NOT used in client packet handler)
    Destroy,
}

impl AiReaction {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::Alert => 0x0,
            Self::Friendly => 0x1,
            Self::Hostile => 0x2,
            Self::Afraid => 0x3,
            Self::Destroy => 0x4,
        }
    }

    pub const fn variants() -> [Self; 5] {
        [
            Self::Alert,
            Self::Friendly,
            Self::Hostile,
            Self::Afraid,
            Self::Destroy,
        ]
    }

    pub const fn from_int(value: u32) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::Alert),
            1 => Ok(Self::Friendly),
            2 => Ok(Self::Hostile),
            3 => Ok(Self::Afraid),
            4 => Ok(Self::Destroy),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl AiReaction {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Alert => "ALERT",
            Self::Friendly => "FRIENDLY",
            Self::Hostile => "HOSTILE",
            Self::Afraid => "AFRAID",
            Self::Destroy => "DESTROY",
        }
    }

}

const NAME: &str = "AiReaction";

impl Default for AiReaction {
    fn default() -> Self {
        Self::Alert
    }
}

impl std::fmt::Display for AiReaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Alert => f.write_str("Alert"),
            Self::Friendly => f.write_str("Friendly"),
            Self::Hostile => f.write_str("Hostile"),
            Self::Afraid => f.write_str("Afraid"),
            Self::Destroy => f.write_str("Destroy"),
        }
    }
}

impl TryFrom<u32> for AiReaction {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u8> for AiReaction {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value.into())
    }
}

impl TryFrom<u16> for AiReaction {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_int(value.into())
    }
}

impl TryFrom<u64> for AiReaction {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for AiReaction {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for AiReaction {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for AiReaction {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let v = u32::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i64> for AiReaction {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for AiReaction {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

