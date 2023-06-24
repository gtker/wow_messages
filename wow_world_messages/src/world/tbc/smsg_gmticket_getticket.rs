use std::io::{Read, Write};

use crate::tbc::{
    GmTicketEscalationStatus, GmTicketStatus, GmTicketType,
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gamemaster/smsg_gmticket_getticket.wowm:15`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/smsg_gmticket_getticket.wowm#L15):
/// ```text
/// smsg SMSG_GMTICKET_GETTICKET = 0x0212 {
///     GmTicketStatus status;
///     if (status == HAS_TEXT) {
///         CString text;
///         GmTicketType ticket_type;
///         f32 days_since_ticket_creation;
///         f32 days_since_oldest_ticket_creation;
///         f32 days_since_last_updated;
///         GmTicketEscalationStatus escalation_status;
///         Bool read_by_gm;
///     }
/// }
/// ```
pub struct SMSG_GMTICKET_GETTICKET {
    pub status: SMSG_GMTICKET_GETTICKET_GmTicketStatus,
}

impl crate::private::Sealed for SMSG_GMTICKET_GETTICKET {}
impl crate::Message for SMSG_GMTICKET_GETTICKET {
    const OPCODE: u32 = 0x0212;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_GMTICKET_GETTICKET {{").unwrap();
        // Members
        writeln!(s, "    status = {};", GmTicketStatus::try_from(self.status.as_int()).unwrap().as_test_case_value()).unwrap();
        match &self.status {
            crate::tbc::SMSG_GMTICKET_GETTICKET_GmTicketStatus::HasText {
                days_since_last_updated,
                days_since_oldest_ticket_creation,
                days_since_ticket_creation,
                escalation_status,
                read_by_gm,
                text,
                ticket_type,
            } => {
                writeln!(s, "    text = \"{}\";", text).unwrap();
                writeln!(s, "    ticket_type = {};", ticket_type.as_test_case_value()).unwrap();
                writeln!(s, "    {}", if days_since_ticket_creation.to_string().contains('.') { days_since_ticket_creation.to_string() } else { format!("{}.0", days_since_ticket_creation) }).unwrap();
                writeln!(s, "    {}", if days_since_oldest_ticket_creation.to_string().contains('.') { days_since_oldest_ticket_creation.to_string() } else { format!("{}.0", days_since_oldest_ticket_creation) }).unwrap();
                writeln!(s, "    {}", if days_since_last_updated.to_string().contains('.') { days_since_last_updated.to_string() } else { format!("{}.0", days_since_last_updated) }).unwrap();
                writeln!(s, "    escalation_status = {};", escalation_status.as_test_case_value()).unwrap();
                writeln!(s, "    read_by_gm = {};", if *read_by_gm { "TRUE" } else { "FALSE" }).unwrap();
            }
            _ => {}
        }


        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 530_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "status", "    ");
        match &self.status {
            crate::tbc::SMSG_GMTICKET_GETTICKET_GmTicketStatus::HasText {
                days_since_last_updated,
                days_since_oldest_ticket_creation,
                days_since_ticket_creation,
                escalation_status,
                read_by_gm,
                text,
                ticket_type,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, text.len() + 1, "text", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "ticket_type", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "days_since_ticket_creation", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "days_since_oldest_ticket_creation", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "days_since_last_updated", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "escalation_status", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "read_by_gm", "    ");
            }
            _ => {}
        }



        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // status: GmTicketStatus
        w.write_all(&(self.status.as_int().to_le_bytes()))?;

        match &self.status {
            SMSG_GMTICKET_GETTICKET_GmTicketStatus::HasText {
                days_since_last_updated,
                days_since_oldest_ticket_creation,
                days_since_ticket_creation,
                escalation_status,
                read_by_gm,
                text,
                ticket_type,
            } => {
                // text: CString
                // TODO: Guard against strings that are already null-terminated
                assert_ne!(text.as_bytes().iter().rev().next(), Some(&0_u8), "String `text` must not be null-terminated.");
                w.write_all(text.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // ticket_type: GmTicketType
                w.write_all(&(ticket_type.as_int().to_le_bytes()))?;

                // days_since_ticket_creation: f32
                w.write_all(&days_since_ticket_creation.to_le_bytes())?;

                // days_since_oldest_ticket_creation: f32
                w.write_all(&days_since_oldest_ticket_creation.to_le_bytes())?;

                // days_since_last_updated: f32
                w.write_all(&days_since_last_updated.to_le_bytes())?;

                // escalation_status: GmTicketEscalationStatus
                w.write_all(&(escalation_status.as_int().to_le_bytes()))?;

                // read_by_gm: Bool
                w.write_all(u8::from(*read_by_gm).to_le_bytes().as_slice())?;

            }
            _ => {}
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(4..=275).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0212, size: body_size });
        }

        // status: GmTicketStatus
        let status = crate::util::read_u32_le(&mut r)?.try_into()?;

        let status_if = match status {
            GmTicketStatus::DbError => SMSG_GMTICKET_GETTICKET_GmTicketStatus::DbError,
            GmTicketStatus::HasText => {
                // text: CString
                let text = {
                    let text = crate::util::read_c_string_to_vec(&mut r)?;
                    String::from_utf8(text)?
                };

                // ticket_type: GmTicketType
                let ticket_type = crate::util::read_u8_le(&mut r)?.try_into()?;

                // days_since_ticket_creation: f32
                let days_since_ticket_creation = crate::util::read_f32_le(&mut r)?;

                // days_since_oldest_ticket_creation: f32
                let days_since_oldest_ticket_creation = crate::util::read_f32_le(&mut r)?;

                // days_since_last_updated: f32
                let days_since_last_updated = crate::util::read_f32_le(&mut r)?;

                // escalation_status: GmTicketEscalationStatus
                let escalation_status = crate::util::read_u8_le(&mut r)?.try_into()?;

                // read_by_gm: Bool
                let read_by_gm = crate::util::read_u8_le(&mut r)? != 0;

                SMSG_GMTICKET_GETTICKET_GmTicketStatus::HasText {
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

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_GMTICKET_GETTICKET {}

impl SMSG_GMTICKET_GETTICKET {
    pub(crate) fn size(&self) -> usize {
        self.status.size() // status: SMSG_GMTICKET_GETTICKET_GmTicketStatus
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum SMSG_GMTICKET_GETTICKET_GmTicketStatus {
    DbError,
    HasText {
        days_since_last_updated: f32,
        days_since_oldest_ticket_creation: f32,
        days_since_ticket_creation: f32,
        escalation_status: GmTicketEscalationStatus,
        read_by_gm: bool,
        text: String,
        ticket_type: GmTicketType,
    },
    Default,
}

impl Default for SMSG_GMTICKET_GETTICKET_GmTicketStatus {
    fn default() -> Self {
        // First enumerator without any fields
        Self::DbError
    }
}

impl SMSG_GMTICKET_GETTICKET_GmTicketStatus {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::DbError => 0,
            Self::HasText { .. } => 6,
            Self::Default => 10,
        }
    }

}

impl std::fmt::Display for SMSG_GMTICKET_GETTICKET_GmTicketStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DbError => f.write_str("DbError"),
            Self::HasText{ .. } => f.write_str("HasText"),
            Self::Default => f.write_str("Default"),
        }
    }
}

impl SMSG_GMTICKET_GETTICKET_GmTicketStatus {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::HasText {
                text,
                ..
            } => {
                4
                + 4 // days_since_last_updated: f32
                + 4 // days_since_oldest_ticket_creation: f32
                + 4 // days_since_ticket_creation: f32
                + 1 // escalation_status: GmTicketEscalationStatus
                + 1 // read_by_gm: Bool
                + text.len() + 1 // text: CString
                + 1 // ticket_type: GmTicketType
            }
            _ => 4,
        }
    }
}

