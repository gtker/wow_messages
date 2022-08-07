use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spellenergizelog.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spellenergizelog.wowm#L3):
/// ```text
/// enum PowerType : u32 {
///     MANA = 0;
///     RAGE = 1;
///     FOCUS = 2;
///     ENERGY = 3;
///     HAPPINESS = 4;
///     HEALTH = 0xFFFFFFFE;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum PowerType {
    /// UNIT_FIELD_POWER1
    ///
    Mana,
    /// UNIT_FIELD_POWER2
    ///
    Rage,
    /// UNIT_FIELD_POWER3
    ///
    Focus,
    /// UNIT_FIELD_POWER4
    ///
    Energy,
    /// UNIT_FIELD_POWER5
    ///
    Happiness,
    /// (-2 as signed value)
    ///
    Health,
}

impl PowerType {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::Mana => 0x0,
            Self::Rage => 0x1,
            Self::Focus => 0x2,
            Self::Energy => 0x3,
            Self::Happiness => 0x4,
            Self::Health => 0xfffffffe,
        }
    }

}

impl Default for PowerType {
    fn default() -> Self {
        Self::Mana
    }
}

impl std::fmt::Display for PowerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mana => f.write_str("Mana"),
            Self::Rage => f.write_str("Rage"),
            Self::Focus => f.write_str("Focus"),
            Self::Energy => f.write_str("Energy"),
            Self::Happiness => f.write_str("Happiness"),
            Self::Health => f.write_str("Health"),
        }
    }
}

impl TryFrom<u32> for PowerType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Mana),
            1 => Ok(Self::Rage),
            2 => Ok(Self::Focus),
            3 => Ok(Self::Energy),
            4 => Ok(Self::Happiness),
            4294967294 => Ok(Self::Health),
            v => Err(crate::errors::EnumError::new("PowerType", v as u32),)
        }
    }
}

