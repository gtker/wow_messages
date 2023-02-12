/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_item_push_result.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_item_push_result.wowm#L8):
/// ```text
/// enum NewItemCreationType : u32 {
///     RECEIVED = 0;
///     CREATED = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum NewItemCreationType {
    Received,
    Created,
}

impl NewItemCreationType {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::Received => 0x0,
            Self::Created => 0x1,
        }
    }

}

impl Default for NewItemCreationType {
    fn default() -> Self {
        Self::Received
    }
}

impl std::fmt::Display for NewItemCreationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Received => f.write_str("Received"),
            Self::Created => f.write_str("Created"),
        }
    }
}

impl TryFrom<u32> for NewItemCreationType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Received),
            1 => Ok(Self::Created),
            v => Err(crate::errors::EnumError::new("NewItemCreationType", v as u64),)
        }
    }
}

