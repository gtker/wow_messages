use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_realm/server.wowm:34`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_realm/server.wowm#L34):
/// ```text
/// enum Population : u32 {
///     GREEN_RECOMMENDED = 0x43480000;
///     RED_FULL = 0x43c80000;
///     BLUE_RECOMMENDED = 0x44160000;
///     OTHER = self.value
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum Population {
    GREEN_RECOMMENDED,
    RED_FULL,
    BLUE_RECOMMENDED,
    OTHER(u32),
}

impl Population {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::GREEN_RECOMMENDED => 0x43480000,
            Self::RED_FULL => 0x43c80000,
            Self::BLUE_RECOMMENDED => 0x44160000,
            Self::OTHER(v) => *v,
        }
    }

}

impl Default for Population {
    fn default() -> Self {
        Self::GREEN_RECOMMENDED
    }
}

impl std::fmt::Display for Population {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GREEN_RECOMMENDED => f.write_str("GREEN_RECOMMENDED"),
            Self::RED_FULL => f.write_str("RED_FULL"),
            Self::BLUE_RECOMMENDED => f.write_str("BLUE_RECOMMENDED"),
            Self::OTHER(v) => f.write_fmt(format_args!("OTHER({})", v)),
        }
    }
}

impl From<u32> for Population {
    fn from(value: u32) -> Self {
        match value {
            1128792064 => Self::GREEN_RECOMMENDED,
            1137180672 => Self::RED_FULL,
            1142292480 => Self::BLUE_RECOMMENDED,
            v => Self::OTHER(v)
        }
    }
}

