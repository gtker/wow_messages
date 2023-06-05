/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/cmsg_battlefield_port.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/cmsg_battlefield_port.wowm#L1):
/// ```text
/// enum BattlefieldPortAction : u8 {
///     LEAVE_QUEUE = 0;
///     ENTER_BATTLE = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum BattlefieldPortAction {
    LeaveQueue,
    EnterBattle,
}

impl BattlefieldPortAction {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::LeaveQueue => 0x0,
            Self::EnterBattle => 0x1,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl BattlefieldPortAction {
    pub fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::LeaveQueue => "LEAVE_QUEUE",
            Self::EnterBattle => "ENTER_BATTLE",
        }
    }

}

impl Default for BattlefieldPortAction {
    fn default() -> Self {
        Self::LeaveQueue
    }
}

impl std::fmt::Display for BattlefieldPortAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LeaveQueue => f.write_str("LeaveQueue"),
            Self::EnterBattle => f.write_str("EnterBattle"),
        }
    }
}

impl TryFrom<u8> for BattlefieldPortAction {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::LeaveQueue),
            1 => Ok(Self::EnterBattle),
            v => Err(crate::errors::EnumError::new("BattlefieldPortAction", v as u64),)
        }
    }
}

