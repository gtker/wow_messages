/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/spell_common.wowm:11`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/spell_common.wowm#L11):
/// ```text
/// enum SpellMissInfo : u8 {
///     NONE = 0;
///     MISS = 1;
///     RESIST = 2;
///     DODGE = 3;
///     PARRY = 4;
///     BLOCK = 5;
///     EVADE = 6;
///     IMMUNE = 7;
///     IMMUNE2 = 8;
///     DEFLECT = 9;
///     ABSORB = 10;
///     REFLECT = 11;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum SpellMissInfo {
    None,
    Miss,
    Resist,
    Dodge,
    Parry,
    Block,
    Evade,
    Immune,
    Immune2,
    Deflect,
    Absorb,
    Reflect,
}

impl SpellMissInfo {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::None => 0x0,
            Self::Miss => 0x1,
            Self::Resist => 0x2,
            Self::Dodge => 0x3,
            Self::Parry => 0x4,
            Self::Block => 0x5,
            Self::Evade => 0x6,
            Self::Immune => 0x7,
            Self::Immune2 => 0x8,
            Self::Deflect => 0x9,
            Self::Absorb => 0xa,
            Self::Reflect => 0xb,
        }
    }

    pub const fn variants() -> [Self; 12] {
        [
            Self::None,
            Self::Miss,
            Self::Resist,
            Self::Dodge,
            Self::Parry,
            Self::Block,
            Self::Evade,
            Self::Immune,
            Self::Immune2,
            Self::Deflect,
            Self::Absorb,
            Self::Reflect,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Miss),
            2 => Ok(Self::Resist),
            3 => Ok(Self::Dodge),
            4 => Ok(Self::Parry),
            5 => Ok(Self::Block),
            6 => Ok(Self::Evade),
            7 => Ok(Self::Immune),
            8 => Ok(Self::Immune2),
            9 => Ok(Self::Deflect),
            10 => Ok(Self::Absorb),
            11 => Ok(Self::Reflect),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl SpellMissInfo {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::None => "NONE",
            Self::Miss => "MISS",
            Self::Resist => "RESIST",
            Self::Dodge => "DODGE",
            Self::Parry => "PARRY",
            Self::Block => "BLOCK",
            Self::Evade => "EVADE",
            Self::Immune => "IMMUNE",
            Self::Immune2 => "IMMUNE2",
            Self::Deflect => "DEFLECT",
            Self::Absorb => "ABSORB",
            Self::Reflect => "REFLECT",
        }
    }

}

const NAME: &str = "SpellMissInfo";

impl Default for SpellMissInfo {
    fn default() -> Self {
        Self::None
    }
}

impl std::fmt::Display for SpellMissInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => f.write_str("None"),
            Self::Miss => f.write_str("Miss"),
            Self::Resist => f.write_str("Resist"),
            Self::Dodge => f.write_str("Dodge"),
            Self::Parry => f.write_str("Parry"),
            Self::Block => f.write_str("Block"),
            Self::Evade => f.write_str("Evade"),
            Self::Immune => f.write_str("Immune"),
            Self::Immune2 => f.write_str("Immune2"),
            Self::Deflect => f.write_str("Deflect"),
            Self::Absorb => f.write_str("Absorb"),
            Self::Reflect => f.write_str("Reflect"),
        }
    }
}

impl TryFrom<u8> for SpellMissInfo {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for SpellMissInfo {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for SpellMissInfo {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for SpellMissInfo {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for SpellMissInfo {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i16> for SpellMissInfo {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for SpellMissInfo {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for SpellMissInfo {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for SpellMissInfo {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

