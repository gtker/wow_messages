/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_procresist.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_procresist.wowm#L3):
/// ```text
/// enum LogFormat : u8 {
///     DEFAULT = 0;
///     DEBUG = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum LogFormat {
    Default,
    Debug,
}

impl LogFormat {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Default => 0x0,
            Self::Debug => 0x1,
        }
    }

}

impl Default for LogFormat {
    fn default() -> Self {
        Self::Default
    }
}

impl std::fmt::Display for LogFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Default => f.write_str("Default"),
            Self::Debug => f.write_str("Debug"),
        }
    }
}

impl TryFrom<u8> for LogFormat {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Default),
            1 => Ok(Self::Debug),
            v => Err(crate::errors::EnumError::new("LogFormat", v as u64),)
        }
    }
}

