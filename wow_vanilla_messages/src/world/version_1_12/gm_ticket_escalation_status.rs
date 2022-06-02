use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gamemaster/smsg_gmticket_getticket.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/smsg_gmticket_getticket.wowm#L3):
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
    GMTICKET_ASSIGNEDTOGM_STATUS_NOT_ASSIGNED,
    /// ticket is assigned to a normal gm
    ///
    GMTICKET_ASSIGNEDTOGM_STATUS_ASSIGNED,
    /// ticket is in the escalation queue
    ///
    GMTICKET_ASSIGNEDTOGM_STATUS_ESCALATED,
}

impl GmTicketEscalationStatus {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::GMTICKET_ASSIGNEDTOGM_STATUS_NOT_ASSIGNED => 0x0,
            Self::GMTICKET_ASSIGNEDTOGM_STATUS_ASSIGNED => 0x1,
            Self::GMTICKET_ASSIGNEDTOGM_STATUS_ESCALATED => 0x2,
        }
    }

}

impl Default for GmTicketEscalationStatus {
    fn default() -> Self {
        Self::GMTICKET_ASSIGNEDTOGM_STATUS_NOT_ASSIGNED
    }
}

impl std::fmt::Display for GmTicketEscalationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GMTICKET_ASSIGNEDTOGM_STATUS_NOT_ASSIGNED => f.write_str("GMTICKET_ASSIGNEDTOGM_STATUS_NOT_ASSIGNED"),
            Self::GMTICKET_ASSIGNEDTOGM_STATUS_ASSIGNED => f.write_str("GMTICKET_ASSIGNEDTOGM_STATUS_ASSIGNED"),
            Self::GMTICKET_ASSIGNEDTOGM_STATUS_ESCALATED => f.write_str("GMTICKET_ASSIGNEDTOGM_STATUS_ESCALATED"),
        }
    }
}

impl TryFrom<u8> for GmTicketEscalationStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::GMTICKET_ASSIGNEDTOGM_STATUS_NOT_ASSIGNED),
            1 => Ok(Self::GMTICKET_ASSIGNEDTOGM_STATUS_ASSIGNED),
            2 => Ok(Self::GMTICKET_ASSIGNEDTOGM_STATUS_ESCALATED),
            v => Err(crate::errors::EnumError::new("GmTicketEscalationStatus", v as u32),)
        }
    }
}

