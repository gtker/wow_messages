/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/smsg_battlefield_status.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/smsg_battlefield_status.wowm#L1):
/// ```text
/// enum StatusId : u8 {
///     NONE = 0;
///     WAIT_QUEUE = 1;
///     WAIT_JOIN = 2;
///     IN_PROGRESS = 3;
///     WAIT_LEAVE = 4;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum StatusId {
    /// first status, should mean bg is not instance
    ///
    None,
    /// means bg is empty and waiting for queue
    ///
    WaitQueue,
    /// this means, that BG has already started and it is waiting for more players
    ///
    WaitJoin,
    /// means bg is running
    ///
    InProgress,
    /// means some faction has won BG and it is ending
    ///
    WaitLeave,
}

impl StatusId {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::None => 0x0,
            Self::WaitQueue => 0x1,
            Self::WaitJoin => 0x2,
            Self::InProgress => 0x3,
            Self::WaitLeave => 0x4,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl StatusId {
    pub fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::None => "NONE",
            Self::WaitQueue => "WAIT_QUEUE",
            Self::WaitJoin => "WAIT_JOIN",
            Self::InProgress => "IN_PROGRESS",
            Self::WaitLeave => "WAIT_LEAVE",
        }
    }

}

impl Default for StatusId {
    fn default() -> Self {
        Self::None
    }
}

impl std::fmt::Display for StatusId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => f.write_str("None"),
            Self::WaitQueue => f.write_str("WaitQueue"),
            Self::WaitJoin => f.write_str("WaitJoin"),
            Self::InProgress => f.write_str("InProgress"),
            Self::WaitLeave => f.write_str("WaitLeave"),
        }
    }
}

impl TryFrom<u8> for StatusId {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::WaitQueue),
            2 => Ok(Self::WaitJoin),
            3 => Ok(Self::InProgress),
            4 => Ok(Self::WaitLeave),
            v => Err(crate::errors::EnumError::new("StatusId", v as u64),)
        }
    }
}

