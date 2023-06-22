/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/smsg_lfg_update.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_lfg_update.wowm#L1):
/// ```text
/// enum LfgUpdateLookingForMore : u8 {
///     NOT_LOOKING_FOR_MORE = 0;
///     LOOKING_FOR_MORE = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum LfgUpdateLookingForMore {
    NotLookingForMore,
    LookingForMore,
}

impl LfgUpdateLookingForMore {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::NotLookingForMore => 0x0,
            Self::LookingForMore => 0x1,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl LfgUpdateLookingForMore {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::NotLookingForMore => "NOT_LOOKING_FOR_MORE",
            Self::LookingForMore => "LOOKING_FOR_MORE",
        }
    }

}

impl Default for LfgUpdateLookingForMore {
    fn default() -> Self {
        Self::NotLookingForMore
    }
}

impl std::fmt::Display for LfgUpdateLookingForMore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotLookingForMore => f.write_str("NotLookingForMore"),
            Self::LookingForMore => f.write_str("LookingForMore"),
        }
    }
}

impl TryFrom<u8> for LfgUpdateLookingForMore {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NotLookingForMore),
            1 => Ok(Self::LookingForMore),
            v => Err(crate::errors::EnumError::new("LfgUpdateLookingForMore", v.into()),)
        }
    }
}

