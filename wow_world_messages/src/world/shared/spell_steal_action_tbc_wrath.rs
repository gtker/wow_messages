use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_spellsteallog.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_spellsteallog.wowm#L1):
/// ```text
/// enum SpellStealAction : u8 {
///     STEAL = 0;
///     CLEANSE = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum SpellStealAction {
    Steal,
    Cleanse,
}

impl SpellStealAction {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Steal => 0x0,
            Self::Cleanse => 0x1,
        }
    }

}

impl Default for SpellStealAction {
    fn default() -> Self {
        Self::Steal
    }
}

impl std::fmt::Display for SpellStealAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Steal => f.write_str("Steal"),
            Self::Cleanse => f.write_str("Cleanse"),
        }
    }
}

impl TryFrom<u8> for SpellStealAction {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Steal),
            1 => Ok(Self::Cleanse),
            v => Err(crate::errors::EnumError::new("SpellStealAction", v as u32),)
        }
    }
}

