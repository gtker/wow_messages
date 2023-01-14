use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_complain_result.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_complain_result.wowm#L1):
/// ```text
/// enum ComplainResultWindow : u8 {
///     DO_NOT_SHOW = 0;
///     SHOW = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum ComplainResultWindow {
    DoNotShow,
    Show,
}

impl ComplainResultWindow {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::DoNotShow => 0x0,
            Self::Show => 0x1,
        }
    }

}

impl Default for ComplainResultWindow {
    fn default() -> Self {
        Self::DoNotShow
    }
}

impl std::fmt::Display for ComplainResultWindow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DoNotShow => f.write_str("DoNotShow"),
            Self::Show => f.write_str("Show"),
        }
    }
}

impl TryFrom<u8> for ComplainResultWindow {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::DoNotShow),
            1 => Ok(Self::Show),
            v => Err(crate::errors::EnumError::new("ComplainResultWindow", v as u64),)
        }
    }
}

