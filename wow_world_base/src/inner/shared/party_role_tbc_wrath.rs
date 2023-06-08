/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/msg_party_assignment.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/msg_party_assignment.wowm#L1):
/// ```text
/// enum PartyRole : u8 {
///     MAIN_TANK = 0;
///     ASSISTANT = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum PartyRole {
    MainTank,
    Assistant,
}

impl PartyRole {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::MainTank => 0x0,
            Self::Assistant => 0x1,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl PartyRole {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::MainTank => "MAIN_TANK",
            Self::Assistant => "ASSISTANT",
        }
    }

}

impl Default for PartyRole {
    fn default() -> Self {
        Self::MainTank
    }
}

impl std::fmt::Display for PartyRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MainTank => f.write_str("MainTank"),
            Self::Assistant => f.write_str("Assistant"),
        }
    }
}

impl TryFrom<u8> for PartyRole {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::MainTank),
            1 => Ok(Self::Assistant),
            v => Err(crate::errors::EnumError::new("PartyRole", v as u64),)
        }
    }
}

