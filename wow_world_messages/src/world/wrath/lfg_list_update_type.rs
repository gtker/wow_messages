use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_update_lfg_list.wowm:12`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_update_lfg_list.wowm#L12):
/// ```text
/// enum LfgListUpdateType : u8 {
///     PARTIAL = 0;
///     FULL = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub(crate) enum LfgListUpdateType {
    Partial,
    Full,
}

impl LfgListUpdateType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Partial => 0x0,
            Self::Full => 0x1,
        }
    }

}

impl Default for LfgListUpdateType {
    fn default() -> Self {
        Self::Partial
    }
}

impl std::fmt::Display for LfgListUpdateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Partial => f.write_str("Partial"),
            Self::Full => f.write_str("Full"),
        }
    }
}

impl TryFrom<u8> for LfgListUpdateType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Partial),
            1 => Ok(Self::Full),
            v => Err(crate::errors::EnumError::new("LfgListUpdateType", v as u32),)
        }
    }
}

