/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_item_push_result.wowm:15`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_item_push_result.wowm#L15):
/// ```text
/// enum NewItemChatAlert : u32 {
///     DO_NOT_SHOW = 0;
///     SHOW = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum NewItemChatAlert {
    DoNotShow,
    Show,
}

impl NewItemChatAlert {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::DoNotShow => 0x0,
            Self::Show => 0x1,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl NewItemChatAlert {
    pub fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::DoNotShow => "DO_NOT_SHOW",
            Self::Show => "SHOW",
        }
    }

}

impl Default for NewItemChatAlert {
    fn default() -> Self {
        Self::DoNotShow
    }
}

impl std::fmt::Display for NewItemChatAlert {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DoNotShow => f.write_str("DoNotShow"),
            Self::Show => f.write_str("Show"),
        }
    }
}

impl TryFrom<u32> for NewItemChatAlert {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::DoNotShow),
            1 => Ok(Self::Show),
            v => Err(crate::errors::EnumError::new("NewItemChatAlert", v as u64),)
        }
    }
}

