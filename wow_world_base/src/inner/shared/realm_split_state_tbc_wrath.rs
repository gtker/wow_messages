/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_realm_split.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_realm_split.wowm#L3):
/// ```text
/// enum RealmSplitState : u32 {
///     NORMAL = 0;
///     SPLIT = 1;
///     SPLIT_PENDING = 2;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum RealmSplitState {
    Normal,
    Split,
    SplitPending,
}

impl RealmSplitState {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::Normal => 0x0,
            Self::Split => 0x1,
            Self::SplitPending => 0x2,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl RealmSplitState {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Normal => "NORMAL",
            Self::Split => "SPLIT",
            Self::SplitPending => "SPLIT_PENDING",
        }
    }

}

impl Default for RealmSplitState {
    fn default() -> Self {
        Self::Normal
    }
}

impl std::fmt::Display for RealmSplitState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => f.write_str("Normal"),
            Self::Split => f.write_str("Split"),
            Self::SplitPending => f.write_str("SplitPending"),
        }
    }
}

impl TryFrom<u32> for RealmSplitState {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Normal),
            1 => Ok(Self::Split),
            2 => Ok(Self::SplitPending),
            v => Err(crate::errors::EnumError::new("RealmSplitState", v.into()),)
        }
    }
}

