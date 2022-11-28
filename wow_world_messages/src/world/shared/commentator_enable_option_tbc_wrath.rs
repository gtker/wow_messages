use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/cmsg_commentator_enable.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/cmsg_commentator_enable.wowm#L1):
/// ```text
/// enum CommentatorEnableOption : u32 {
///     DISABLE = 0;
///     ENABLE = 1;
///     TOGGLE = 2;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum CommentatorEnableOption {
    Disable,
    Enable,
    Toggle,
}

impl CommentatorEnableOption {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::Disable => 0x0,
            Self::Enable => 0x1,
            Self::Toggle => 0x2,
        }
    }

}

impl Default for CommentatorEnableOption {
    fn default() -> Self {
        Self::Disable
    }
}

impl std::fmt::Display for CommentatorEnableOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Disable => f.write_str("Disable"),
            Self::Enable => f.write_str("Enable"),
            Self::Toggle => f.write_str("Toggle"),
        }
    }
}

impl TryFrom<u32> for CommentatorEnableOption {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Disable),
            1 => Ok(Self::Enable),
            2 => Ok(Self::Toggle),
            v => Err(crate::errors::EnumError::new("CommentatorEnableOption", v as u32),)
        }
    }
}

