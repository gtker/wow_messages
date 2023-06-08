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
    GreenRecommended,
    RedFull,
    BlueRecommended,
    Other(u32),
}

impl Population {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::GreenRecommended => 0x43480000,
            Self::RedFull => 0x43c80000,
            Self::BlueRecommended => 0x44160000,
            Self::Other(v) => *v,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl Population {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::GreenRecommended => "GREEN_RECOMMENDED",
            Self::RedFull => "RED_FULL",
            Self::BlueRecommended => "BLUE_RECOMMENDED",
            Self::Other(_) => "OTHER",
        }
    }

}

impl Default for Population {
    fn default() -> Self {
        Self::GreenRecommended
    }
}

impl std::fmt::Display for Population {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GreenRecommended => f.write_str("GreenRecommended"),
            Self::RedFull => f.write_str("RedFull"),
            Self::BlueRecommended => f.write_str("BlueRecommended"),
            Self::Other(v) => f.write_fmt(format_args!("Other({v})")),
        }
    }
}

impl From<u32> for Population {
    fn from(value: u32) -> Self {
        match value {
            1128792064 => Self::GreenRecommended,
            1137180672 => Self::RedFull,
            1142292480 => Self::BlueRecommended,
            v => Self::Other(v)
        }
    }
}

