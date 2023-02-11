/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_use_item.wowm:30`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_use_item.wowm#L30):
/// ```text
/// enum ClientMovementData : u8 {
///     NOT_PRESENT = 0;
///     PRESENT = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub(crate) enum ClientMovementData {
    NotPresent,
    Present,
}

impl ClientMovementData {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NotPresent => 0x0,
            Self::Present => 0x1,
        }
    }

}

impl Default for ClientMovementData {
    fn default() -> Self {
        Self::NotPresent
    }
}

impl std::fmt::Display for ClientMovementData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotPresent => f.write_str("NotPresent"),
            Self::Present => f.write_str("Present"),
        }
    }
}

impl TryFrom<u8> for ClientMovementData {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NotPresent),
            1 => Ok(Self::Present),
            v => Err(crate::errors::EnumError::new("ClientMovementData", v as u64),)
        }
    }
}

