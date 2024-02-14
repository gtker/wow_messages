/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/spell_common.wowm:35`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/spell_common.wowm#L35):
/// ```text
/// enum SpellSchool : u8 {
///     NORMAL = 0;
///     HOLY = 1;
///     FIRE = 2;
///     NATURE = 3;
///     FROST = 4;
///     SHADOW = 5;
///     ARCANE = 6;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum SpellSchool {
    /// Physical, Armor
    Normal,
    Holy,
    Fire,
    Nature,
    Frost,
    Shadow,
    Arcane,
}

impl SpellSchool {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Normal => 0x0,
            Self::Holy => 0x1,
            Self::Fire => 0x2,
            Self::Nature => 0x3,
            Self::Frost => 0x4,
            Self::Shadow => 0x5,
            Self::Arcane => 0x6,
        }
    }

    pub const fn variants() -> [Self; 7] {
        [
            Self::Normal,
            Self::Holy,
            Self::Fire,
            Self::Nature,
            Self::Frost,
            Self::Shadow,
            Self::Arcane,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::Normal),
            1 => Ok(Self::Holy),
            2 => Ok(Self::Fire),
            3 => Ok(Self::Nature),
            4 => Ok(Self::Frost),
            5 => Ok(Self::Shadow),
            6 => Ok(Self::Arcane),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl SpellSchool {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Normal => "NORMAL",
            Self::Holy => "HOLY",
            Self::Fire => "FIRE",
            Self::Nature => "NATURE",
            Self::Frost => "FROST",
            Self::Shadow => "SHADOW",
            Self::Arcane => "ARCANE",
        }
    }

}

const NAME: &str = "SpellSchool";

impl Default for SpellSchool {
    fn default() -> Self {
        Self::Normal
    }
}

impl std::fmt::Display for SpellSchool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => f.write_str("Normal"),
            Self::Holy => f.write_str("Holy"),
            Self::Fire => f.write_str("Fire"),
            Self::Nature => f.write_str("Nature"),
            Self::Frost => f.write_str("Frost"),
            Self::Shadow => f.write_str("Shadow"),
            Self::Arcane => f.write_str("Arcane"),
        }
    }
}

impl TryFrom<u8> for SpellSchool {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for SpellSchool {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for SpellSchool {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for SpellSchool {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for SpellSchool {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i16> for SpellSchool {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for SpellSchool {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for SpellSchool {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for SpellSchool {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

