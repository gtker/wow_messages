/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_transfer_aborted.wowm:38`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_transfer_aborted.wowm#L38):
/// ```text
/// enum DungeonDifficulty : u8 {
///     NORMAL = 0;
///     HEROIC = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum DungeonDifficulty {
    Normal,
    Heroic,
}

impl DungeonDifficulty {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Normal => 0x0,
            Self::Heroic => 0x1,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl DungeonDifficulty {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Normal => "NORMAL",
            Self::Heroic => "HEROIC",
        }
    }

}

impl Default for DungeonDifficulty {
    fn default() -> Self {
        Self::Normal
    }
}

impl std::fmt::Display for DungeonDifficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => f.write_str("Normal"),
            Self::Heroic => f.write_str("Heroic"),
        }
    }
}

impl TryFrom<u8> for DungeonDifficulty {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Normal),
            1 => Ok(Self::Heroic),
            v => Err(crate::errors::EnumError::new("DungeonDifficulty", v as u64),)
        }
    }
}

