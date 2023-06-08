/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/exp/smsg_log_xpgain.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/exp/smsg_log_xpgain.wowm#L1):
/// ```text
/// enum ExperienceAwardType : u8 {
///     KILL = 0;
///     NON_KILL = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum ExperienceAwardType {
    Kill,
    NonKill,
}

impl ExperienceAwardType {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Kill => 0x0,
            Self::NonKill => 0x1,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl ExperienceAwardType {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Kill => "KILL",
            Self::NonKill => "NON_KILL",
        }
    }

}

impl Default for ExperienceAwardType {
    fn default() -> Self {
        Self::Kill
    }
}

impl std::fmt::Display for ExperienceAwardType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Kill => f.write_str("Kill"),
            Self::NonKill => f.write_str("NonKill"),
        }
    }
}

impl TryFrom<u8> for ExperienceAwardType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Kill),
            1 => Ok(Self::NonKill),
            v => Err(crate::errors::EnumError::new("ExperienceAwardType", v as u64),)
        }
    }
}

