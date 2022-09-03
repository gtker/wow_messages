use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::GmTicketEscalationStatus;
use crate::world::vanilla::GmTicketStatus;
use crate::world::vanilla::GmTicketType;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gamemaster/smsg_gmticket_getticket.wowm:15`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/smsg_gmticket_getticket.wowm#L15):
/// ```text
/// smsg SMSG_GMTICKET_GETTICKET = 0x0212 {
///     GmTicketStatus status;
///     if (status == HASTEXT) {
///         CString text;
///         GmTicketType ticket_type;
///         f32 days_since_ticket_creation;
///         f32 days_since_oldest_ticket_creation;
///         f32 days_since_last_updated;
///         GmTicketEscalationStatus escalation_status;
///         u8 read_by_gm;
///     }
/// }
/// ```
pub struct SMSG_GMTICKET_GETTICKET {
    pub status: SMSG_GMTICKET_GETTICKET_GmTicketStatus,
}

impl crate::Message for SMSG_GMTICKET_GETTICKET {
    const OPCODE: u32 = 0x0212;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // status: GmTicketStatus
        w.write_all(&(self.status.as_int() as u32).to_le_bytes())?;

        match &self.status {
            SMSG_GMTICKET_GETTICKET_GmTicketStatus::Dberror => {}
            SMSG_GMTICKET_GETTICKET_GmTicketStatus::Hastext {
                days_since_last_updated,
                days_since_oldest_ticket_creation,
                days_since_ticket_creation,
                escalation_status,
                read_by_gm,
                text,
                ticket_type,
            } => {
                // text: CString
                w.write_all(text.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // ticket_type: GmTicketType
                w.write_all(&(ticket_type.as_int() as u8).to_le_bytes())?;

                // days_since_ticket_creation: f32
                w.write_all(&days_since_ticket_creation.to_le_bytes())?;

                // days_since_oldest_ticket_creation: f32
                w.write_all(&days_since_oldest_ticket_creation.to_le_bytes())?;

                // days_since_last_updated: f32
                w.write_all(&days_since_last_updated.to_le_bytes())?;

                // escalation_status: GmTicketEscalationStatus
                w.write_all(&(escalation_status.as_int() as u8).to_le_bytes())?;

                // read_by_gm: u8
                w.write_all(&read_by_gm.to_le_bytes())?;

            }
            SMSG_GMTICKET_GETTICKET_GmTicketStatus::Default => {}
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // status: GmTicketStatus
        let status: GmTicketStatus = crate::util::read_u32_le(r)?.try_into()?;

        let status_if = match status {
            GmTicketStatus::Dberror => SMSG_GMTICKET_GETTICKET_GmTicketStatus::Dberror,
            GmTicketStatus::Hastext => {
                // text: CString
                let text = crate::util::read_c_string_to_vec(r)?;
                let text = String::from_utf8(text)?;

                // ticket_type: GmTicketType
                let ticket_type: GmTicketType = crate::util::read_u8_le(r)?.try_into()?;

                // days_since_ticket_creation: f32
                let days_since_ticket_creation = crate::util::read_f32_le(r)?;
                // days_since_oldest_ticket_creation: f32
                let days_since_oldest_ticket_creation = crate::util::read_f32_le(r)?;
                // days_since_last_updated: f32
                let days_since_last_updated = crate::util::read_f32_le(r)?;
                // escalation_status: GmTicketEscalationStatus
                let escalation_status: GmTicketEscalationStatus = crate::util::read_u8_le(r)?.try_into()?;

                // read_by_gm: u8
                let read_by_gm = crate::util::read_u8_le(r)?;

                SMSG_GMTICKET_GETTICKET_GmTicketStatus::Hastext {
                    days_since_last_updated,
                    days_since_oldest_ticket_creation,
                    days_since_ticket_creation,
                    escalation_status,
                    read_by_gm,
                    text,
                    ticket_type,
                }
            }
            GmTicketStatus::Default => SMSG_GMTICKET_GETTICKET_GmTicketStatus::Default,
        };

        Ok(Self {
            status: status_if,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_GMTICKET_GETTICKET {}

impl SMSG_GMTICKET_GETTICKET {
    pub(crate) fn size(&self) -> usize {
        self.status.size() // status: SMSG_GMTICKET_GETTICKET_GmTicketStatus
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SMSG_GMTICKET_GETTICKET_GmTicketStatus {
    Dberror,
    Hastext {
        days_since_last_updated: f32,
        days_since_oldest_ticket_creation: f32,
        days_since_ticket_creation: f32,
        escalation_status: GmTicketEscalationStatus,
        read_by_gm: u8,
        text: String,
        ticket_type: GmTicketType,
    },
    Default,
}

impl Default for SMSG_GMTICKET_GETTICKET_GmTicketStatus {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Dberror
    }
}

impl SMSG_GMTICKET_GETTICKET_GmTicketStatus {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::Dberror => 0,
            Self::Hastext { .. } => 6,
            Self::Default => 10,
        }
    }

}

impl SMSG_GMTICKET_GETTICKET_GmTicketStatus {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Dberror => {
                4
            }
            Self::Hastext {
                days_since_last_updated,
                days_since_oldest_ticket_creation,
                days_since_ticket_creation,
                escalation_status,
                read_by_gm,
                text,
                ticket_type,
            } => {
                4
                + 4 // days_since_last_updated: f32
                + 4 // days_since_oldest_ticket_creation: f32
                + 4 // days_since_ticket_creation: f32
                + 1 // escalation_status: GmTicketEscalationStatus
                + 1 // read_by_gm: u8
                + text.len() + 1 // text: CString
                + 1 // ticket_type: GmTicketType
            }
            Self::Default => {
                4
            }
        }
    }
}

