/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spellsteallog.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spellsteallog.wowm#L1):
/// ```text
/// enum SpellStealAction : u8 {
///     STEAL = 0;
///     CLEANSE = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum SpellStealAction {
    Steal,
    Cleanse,
}

impl SpellStealAction {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Steal => 0x0,
            Self::Cleanse => 0x1,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl SpellStealAction {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Steal => "STEAL",
            Self::Cleanse => "CLEANSE",
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
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Steal),
            1 => Ok(Self::Cleanse),
            v => Err(crate::errors::EnumError::new("SpellStealAction", v as u64),)
        }
    }
}

