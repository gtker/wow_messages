/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_addon_info.wowm:13`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_addon_info.wowm#L13):
/// ```text
/// enum InfoBlock : u8 {
///     UNAVAILABLE = 0;
///     AVAILABLE = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum InfoBlock {
    Unavailable,
    Available,
}

impl InfoBlock {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Unavailable => 0x0,
            Self::Available => 0x1,
        }
    }

}

impl Default for InfoBlock {
    fn default() -> Self {
        Self::Unavailable
    }
}

impl std::fmt::Display for InfoBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unavailable => f.write_str("Unavailable"),
            Self::Available => f.write_str("Available"),
        }
    }
}

impl TryFrom<u8> for InfoBlock {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Unavailable),
            1 => Ok(Self::Available),
            v => Err(crate::errors::EnumError::new("InfoBlock", v as u64),)
        }
    }
}

