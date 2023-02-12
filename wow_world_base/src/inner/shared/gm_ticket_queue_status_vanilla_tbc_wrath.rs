/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gamemaster/smsg_gmticket_systemstatus.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/smsg_gmticket_systemstatus.wowm#L3):
/// ```text
/// enum GmTicketQueueStatus : u32 {
///     ENABLED = 1;
///     DISABLED = 0;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum GmTicketQueueStatus {
    Enabled,
    Disabled,
}

impl GmTicketQueueStatus {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::Enabled => 0x1,
            Self::Disabled => 0x0,
        }
    }

}

impl Default for GmTicketQueueStatus {
    fn default() -> Self {
        Self::Enabled
    }
}

impl std::fmt::Display for GmTicketQueueStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Enabled => f.write_str("Enabled"),
            Self::Disabled => f.write_str("Disabled"),
        }
    }
}

impl TryFrom<u32> for GmTicketQueueStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Enabled),
            0 => Ok(Self::Disabled),
            v => Err(crate::errors::EnumError::new("GmTicketQueueStatus", v as u64),)
        }
    }
}

