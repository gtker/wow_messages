/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/loot_common.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/loot_common.wowm#L1):
/// ```text
/// enum RollVote : u8 {
///     PASS = 0;
///     NEED = 1;
///     GREED = 2;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum RollVote {
    Pass,
    Need,
    Greed,
}

impl RollVote {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Pass => 0x0,
            Self::Need => 0x1,
            Self::Greed => 0x2,
        }
    }

}

impl Default for RollVote {
    fn default() -> Self {
        Self::Pass
    }
}

impl std::fmt::Display for RollVote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pass => f.write_str("Pass"),
            Self::Need => f.write_str("Need"),
            Self::Greed => f.write_str("Greed"),
        }
    }
}

impl TryFrom<u8> for RollVote {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Pass),
            1 => Ok(Self::Need),
            2 => Ok(Self::Greed),
            v => Err(crate::errors::EnumError::new("RollVote", v as u64),)
        }
    }
}

