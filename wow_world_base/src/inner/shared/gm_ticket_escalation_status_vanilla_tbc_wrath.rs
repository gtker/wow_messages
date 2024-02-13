/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gamemaster/smsg_gmticket_getticket.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/smsg_gmticket_getticket.wowm#L1):
/// ```text
/// enum GmTicketEscalationStatus : u8 {
///     GMTICKET_ASSIGNEDTOGM_STATUS_NOT_ASSIGNED = 0;
///     GMTICKET_ASSIGNEDTOGM_STATUS_ASSIGNED = 1;
///     GMTICKET_ASSIGNEDTOGM_STATUS_ESCALATED = 2;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum GmTicketEscalationStatus {
    /// ticket is not currently assigned to a gm
    GmticketAssignedtogmStatusNotAssigned,
    /// ticket is assigned to a normal gm
    GmticketAssignedtogmStatusAssigned,
    /// ticket is in the escalation queue
    GmticketAssignedtogmStatusEscalated,
}

impl GmTicketEscalationStatus {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::GmticketAssignedtogmStatusNotAssigned => 0x0,
            Self::GmticketAssignedtogmStatusAssigned => 0x1,
            Self::GmticketAssignedtogmStatusEscalated => 0x2,
        }
    }

    pub const fn variants() -> [Self; 3] {
        [
            Self::GmticketAssignedtogmStatusNotAssigned,
            Self::GmticketAssignedtogmStatusAssigned,
            Self::GmticketAssignedtogmStatusEscalated,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::GmticketAssignedtogmStatusNotAssigned),
            1 => Ok(Self::GmticketAssignedtogmStatusAssigned),
            2 => Ok(Self::GmticketAssignedtogmStatusEscalated),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl GmTicketEscalationStatus {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::GmticketAssignedtogmStatusNotAssigned => "GMTICKET_ASSIGNEDTOGM_STATUS_NOT_ASSIGNED",
            Self::GmticketAssignedtogmStatusAssigned => "GMTICKET_ASSIGNEDTOGM_STATUS_ASSIGNED",
            Self::GmticketAssignedtogmStatusEscalated => "GMTICKET_ASSIGNEDTOGM_STATUS_ESCALATED",
        }
    }

}

const NAME: &str = "GmTicketEscalationStatus";

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
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for GmTicketEscalationStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for GmTicketEscalationStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for GmTicketEscalationStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for GmTicketEscalationStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i16> for GmTicketEscalationStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for GmTicketEscalationStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for GmTicketEscalationStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for GmTicketEscalationStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

