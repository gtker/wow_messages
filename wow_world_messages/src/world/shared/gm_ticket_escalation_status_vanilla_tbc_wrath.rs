use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gamemaster/smsg_gmticket_getticket.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/smsg_gmticket_getticket.wowm#L1):
/// ```text
/// enum GmTicketEscalationStatus : u8 {
///     GMTICKET_ASSIGNEDTOGM_STATUS_NOT_ASSIGNED = 0;
///     GMTICKET_ASSIGNEDTOGM_STATUS_ASSIGNED = 1;
///     GMTICKET_ASSIGNEDTOGM_STATUS_ESCALATED = 2;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum GmTicketEscalationStatus {
    /// ticket is not currently assigned to a gm
    ///
    GmticketAssignedtogmStatusNotAssigned,
    /// ticket is assigned to a normal gm
    ///
    GmticketAssignedtogmStatusAssigned,
    /// ticket is in the escalation queue
    ///
    GmticketAssignedtogmStatusEscalated,
}

impl GmTicketEscalationStatus {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::GmticketAssignedtogmStatusNotAssigned => 0x0,
            Self::GmticketAssignedtogmStatusAssigned => 0x1,
            Self::GmticketAssignedtogmStatusEscalated => 0x2,
        }
    }

}

impl Default for GmTicketEscalationStatus {
    fn default() -> Self {
        Self::GmticketAssignedtogmStatusNotAssigned
    }
}

impl std::fmt::Display for GmTicketEscalationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GmticketAssignedtogmStatusNotAssigned => f.write_str("GmticketAssignedtogmStatusNotAssigned"),
            Self::GmticketAssignedtogmStatusAssigned => f.write_str("GmticketAssignedtogmStatusAssigned"),
            Self::GmticketAssignedtogmStatusEscalated => f.write_str("GmticketAssignedtogmStatusEscalated"),
        }
    }
}

impl TryFrom<u8> for GmTicketEscalationStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::GmticketAssignedtogmStatusNotAssigned),
            1 => Ok(Self::GmticketAssignedtogmStatusAssigned),
            2 => Ok(Self::GmticketAssignedtogmStatusEscalated),
            v => Err(crate::errors::EnumError::new("GmTicketEscalationStatus", v as u64),)
        }
    }
}

