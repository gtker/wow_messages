/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_party_command_result.wowm:32`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_party_command_result.wowm#L32):
/// ```text
/// enum PartyOperation : u8 {
///     INVITE = 0;
///     LEAVE = 2;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum PartyOperation {
    Invite,
    Leave,
}

impl PartyOperation {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Invite => 0x0,
            Self::Leave => 0x2,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl PartyOperation {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Invite => "INVITE",
            Self::Leave => "LEAVE",
        }
    }

}

impl Default for PartyOperation {
    fn default() -> Self {
        Self::Invite
    }
}

impl std::fmt::Display for PartyOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invite => f.write_str("Invite"),
            Self::Leave => f.write_str("Leave"),
        }
    }
}

impl TryFrom<u8> for PartyOperation {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Invite),
            2 => Ok(Self::Leave),
            v => Err(crate::errors::EnumError::new("PartyOperation", v as u64),)
        }
    }
}

