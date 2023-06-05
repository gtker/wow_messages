/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_title_earned.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_title_earned.wowm#L1):
/// ```text
/// enum TitleEarnStatus : u32 {
///     LOST = 0;
///     EARNED = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum TitleEarnStatus {
    Lost,
    Earned,
}

impl TitleEarnStatus {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::Lost => 0x0,
            Self::Earned => 0x1,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl TitleEarnStatus {
    pub fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Lost => "LOST",
            Self::Earned => "EARNED",
        }
    }

}

impl Default for TitleEarnStatus {
    fn default() -> Self {
        Self::Lost
    }
}

impl std::fmt::Display for TitleEarnStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lost => f.write_str("Lost"),
            Self::Earned => f.write_str("Earned"),
        }
    }
}

impl TryFrom<u32> for TitleEarnStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Lost),
            1 => Ok(Self::Earned),
            v => Err(crate::errors::EnumError::new("TitleEarnStatus", v as u64),)
        }
    }
}

