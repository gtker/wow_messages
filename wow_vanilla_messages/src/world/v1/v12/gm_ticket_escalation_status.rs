use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum GmTicketEscalationStatus {
    GMTICKET_ASSIGNEDTOGM_STATUS_NOT_ASSIGNED,
    GMTICKET_ASSIGNEDTOGM_STATUS_ASSIGNED,
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
    type Error = GmTicketEscalationStatusError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::GMTICKET_ASSIGNEDTOGM_STATUS_NOT_ASSIGNED),
            1 => Ok(Self::GMTICKET_ASSIGNEDTOGM_STATUS_ASSIGNED),
            2 => Ok(Self::GMTICKET_ASSIGNEDTOGM_STATUS_ESCALATED),
            _ => Err(GmTicketEscalationStatusError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct GmTicketEscalationStatusError {
    pub value: u8,
}

impl GmTicketEscalationStatusError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for GmTicketEscalationStatusError {}
impl std::fmt::Display for GmTicketEscalationStatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'GmTicketEscalationStatus': '{}'", self.value))
    }
}

