/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/cmsg_lfg_teleport.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/cmsg_lfg_teleport.wowm#L1):
/// ```text
/// enum LfgTeleportLocation : u8 {
///     IN = 0;
///     OUT = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum LfgTeleportLocation {
    In,
    Out,
}

impl LfgTeleportLocation {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::In => 0x0,
            Self::Out => 0x1,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl LfgTeleportLocation {
    pub fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::In => "IN",
            Self::Out => "OUT",
        }
    }

}

impl Default for LfgTeleportLocation {
    fn default() -> Self {
        Self::In
    }
}

impl std::fmt::Display for LfgTeleportLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::In => f.write_str("In"),
            Self::Out => f.write_str("Out"),
        }
    }
}

impl TryFrom<u8> for LfgTeleportLocation {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::In),
            1 => Ok(Self::Out),
            v => Err(crate::errors::EnumError::new("LfgTeleportLocation", v as u64),)
        }
    }
}

