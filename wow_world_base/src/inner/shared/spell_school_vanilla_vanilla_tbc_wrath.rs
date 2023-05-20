/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/spell_common.wowm:36`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/spell_common.wowm#L36):
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
    ///
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

}

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
        match value {
            0 => Ok(Self::Normal),
            1 => Ok(Self::Holy),
            2 => Ok(Self::Fire),
            3 => Ok(Self::Nature),
            4 => Ok(Self::Frost),
            5 => Ok(Self::Shadow),
            6 => Ok(Self::Arcane),
            v => Err(crate::errors::EnumError::new("SpellSchool", v as u64),)
        }
    }
}

