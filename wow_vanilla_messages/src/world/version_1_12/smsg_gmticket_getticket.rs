use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::GmTicketEscalationStatus;
use crate::world::version_1_12::GmTicketStatus;
use crate::world::version_1_12::GmTicketType;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
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

impl ServerMessage for SMSG_GMTICKET_GETTICKET {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // status: GmTicketStatus
        w.write_all(&(self.status.as_int() as u32).to_le_bytes())?;

        match &self.status {
            SMSG_GMTICKET_GETTICKET_GmTicketStatus::DBERROR => {}
            SMSG_GMTICKET_GETTICKET_GmTicketStatus::HASTEXT {
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
            SMSG_GMTICKET_GETTICKET_GmTicketStatus::DEFAULT => {}
        }

        Ok(())
    }
    const OPCODE: u16 = 0x0212;

    fn server_size(&self) -> u16 {
        (self.size() + 4) as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // status: GmTicketStatus
        let status: GmTicketStatus = crate::util::read_u32_le(r)?.try_into()?;

        let status_if = match status {
            GmTicketStatus::DBERROR => SMSG_GMTICKET_GETTICKET_GmTicketStatus::DBERROR,
            GmTicketStatus::HASTEXT => {
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

                SMSG_GMTICKET_GETTICKET_GmTicketStatus::HASTEXT {
                    days_since_last_updated,
                    days_since_oldest_ticket_creation,
                    days_since_ticket_creation,
                    escalation_status,
                    read_by_gm,
                    text,
                    ticket_type,
                }
            }
            GmTicketStatus::DEFAULT => SMSG_GMTICKET_GETTICKET_GmTicketStatus::DEFAULT,
        };

        Ok(Self {
            status: status_if,
        })
    }

}

impl SMSG_GMTICKET_GETTICKET {
    pub(crate) fn size(&self) -> usize {
        self.status.size() // status: SMSG_GMTICKET_GETTICKET_GmTicketStatus
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SMSG_GMTICKET_GETTICKET_GmTicketStatus {
    DBERROR,
    HASTEXT {
        days_since_last_updated: f32,
        days_since_oldest_ticket_creation: f32,
        days_since_ticket_creation: f32,
        escalation_status: GmTicketEscalationStatus,
        read_by_gm: u8,
        text: String,
        ticket_type: GmTicketType,
    },
    DEFAULT,
}

impl Default for SMSG_GMTICKET_GETTICKET_GmTicketStatus {
    fn default() -> Self {
        // First enumerator without any fields
        Self::DBERROR
    }
}

impl SMSG_GMTICKET_GETTICKET_GmTicketStatus {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::DBERROR => 0,
            Self::HASTEXT { .. } => 6,
            Self::DEFAULT => 10,
        }
    }

}

impl SMSG_GMTICKET_GETTICKET_GmTicketStatus {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::DBERROR => {
                4
            }
            Self::HASTEXT {
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
            Self::DEFAULT => {
                4
            }
        }
    }
}

