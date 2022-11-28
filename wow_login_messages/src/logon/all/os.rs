use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/challenge_client_commons.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/challenge_client_commons.wowm#L3):
/// ```text
/// enum Os : u32 {
///     WINDOWS = "\0Win";
///     MAC_OS_X = "\0OSX";
///     OTHER = self.value
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum Os {
    Windows,
    MacOsX,
    Other(u32),
}

impl Os {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::Windows => 0x57696e,
            Self::MacOsX => 0x4f5358,
            Self::Other(v) => *v,
        }
    }

}

impl Default for Os {
    fn default() -> Self {
        Self::Windows
    }
}

impl std::fmt::Display for Os {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Windows => f.write_str("Windows"),
            Self::MacOsX => f.write_str("MacOsX"),
            Self::Other(v) => f.write_fmt(format_args!("Other({})", v)),
        }
    }
}

impl From<u32> for Os {
    fn from(value: u32) -> Self {
        match value {
            5728622 => Self::Windows,
            5198680 => Self::MacOsX,
            v => Self::Other(v)
        }
    }
}

