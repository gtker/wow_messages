use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_item_push_result.wowm:13`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_item_push_result.wowm#L13):
/// ```text
/// enum NewItemChatAlert : u32 {
///     DO_NOT_SHOW = 0;
///     SHOW = 1;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum NewItemChatAlert {
    DO_NOT_SHOW,
    SHOW,
}

impl NewItemChatAlert {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::DO_NOT_SHOW => 0x0,
            Self::SHOW => 0x1,
        }
    }

}

impl Default for NewItemChatAlert {
    fn default() -> Self {
        Self::DO_NOT_SHOW
    }
}

impl std::fmt::Display for NewItemChatAlert {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DO_NOT_SHOW => f.write_str("DO_NOT_SHOW"),
            Self::SHOW => f.write_str("SHOW"),
        }
    }
}

impl TryFrom<u32> for NewItemChatAlert {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::DO_NOT_SHOW),
            1 => Ok(Self::SHOW),
            v => Err(crate::errors::EnumError::new("NewItemChatAlert", v as u32),)
        }
    }
}

