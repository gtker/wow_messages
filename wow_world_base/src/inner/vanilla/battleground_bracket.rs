/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/smsg_battlefield_list.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/smsg_battlefield_list.wowm#L1):
/// ```text
/// enum BattlegroundBracket : u8 {
///     TENS = 0;
///     TWENTIES = 1;
///     THIRTIES = 2;
///     FOURTIES = 3;
///     FIFTIES = 4;
///     SIXTY = 5;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum BattlegroundBracket {
    /// 10-19
    Tens,
    /// 20-29
    Twenties,
    /// 30-39
    Thirties,
    /// 40-49
    Fourties,
    /// 50-59
    Fifties,
    /// 60
    Sixty,
}

impl BattlegroundBracket {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Tens => 0x0,
            Self::Twenties => 0x1,
            Self::Thirties => 0x2,
            Self::Fourties => 0x3,
            Self::Fifties => 0x4,
            Self::Sixty => 0x5,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl BattlegroundBracket {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Tens => "TENS",
            Self::Twenties => "TWENTIES",
            Self::Thirties => "THIRTIES",
            Self::Fourties => "FOURTIES",
            Self::Fifties => "FIFTIES",
            Self::Sixty => "SIXTY",
        }
    }

}

impl Default for BattlegroundBracket {
    fn default() -> Self {
        Self::Tens
    }
}

impl std::fmt::Display for BattlegroundBracket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tens => f.write_str("Tens"),
            Self::Twenties => f.write_str("Twenties"),
            Self::Thirties => f.write_str("Thirties"),
            Self::Fourties => f.write_str("Fourties"),
            Self::Fifties => f.write_str("Fifties"),
            Self::Sixty => f.write_str("Sixty"),
        }
    }
}

impl TryFrom<u8> for BattlegroundBracket {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Tens),
            1 => Ok(Self::Twenties),
            2 => Ok(Self::Thirties),
            3 => Ok(Self::Fourties),
            4 => Ok(Self::Fifties),
            5 => Ok(Self::Sixty),
            v => Err(crate::errors::EnumError::new("BattlegroundBracket", v.into()),)
        }
    }
}

