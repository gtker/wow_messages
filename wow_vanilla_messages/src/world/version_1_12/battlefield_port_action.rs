use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/cmsg_battlefield_port.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/cmsg_battlefield_port.wowm#L3):
/// ```text
/// enum BattlefieldPortAction : u8 {
///     LEAVE_QUEUE = 0;
///     ENTER_BATTLE = 1;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum BattlefieldPortAction {
    LEAVE_QUEUE,
    ENTER_BATTLE,
}

impl BattlefieldPortAction {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::LEAVE_QUEUE => 0x0,
            Self::ENTER_BATTLE => 0x1,
        }
    }

}

impl Default for BattlefieldPortAction {
    fn default() -> Self {
        Self::LEAVE_QUEUE
    }
}

impl std::fmt::Display for BattlefieldPortAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LEAVE_QUEUE => f.write_str("LEAVE_QUEUE"),
            Self::ENTER_BATTLE => f.write_str("ENTER_BATTLE"),
        }
    }
}

impl TryFrom<u8> for BattlefieldPortAction {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::LEAVE_QUEUE),
            1 => Ok(Self::ENTER_BATTLE),
            v => Err(crate::errors::EnumError::new("BattlefieldPortAction", v as u32),)
        }
    }
}

