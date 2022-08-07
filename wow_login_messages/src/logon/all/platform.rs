use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/challenge_client_commons.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/challenge_client_commons.wowm#L9):
/// ```text
/// enum Platform : u32 {
///     X86 = "\0x86";
///     POWER_PC = "\0PPC";
///     OTHER = self.value
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum Platform {
    X86,
    PowerPc,
    Other(u32),
}

impl Platform {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::X86 => 0x783836,
            Self::PowerPc => 0x505043,
            Self::Other(v) => *v,
        }
    }

}

impl Default for Platform {
    fn default() -> Self {
        Self::X86
    }
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::X86 => f.write_str("X86"),
            Self::PowerPc => f.write_str("PowerPc"),
            Self::Other(v) => f.write_fmt(format_args!("Other({})", v)),
        }
    }
}

impl From<u32> for Platform {
    fn from(value: u32) -> Self {
        match value {
            7878710 => Self::X86,
            5263427 => Self::PowerPc,
            v => Self::Other(v)
        }
    }
}

