use std::io::{Read, Write};

use crate::{
    DateTime, Guid,
};
use crate::shared::level_vanilla_tbc_wrath::Level;
use crate::wrath::CalendarStatusTime;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/smsg_calendar_event_invite.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/smsg_calendar_event_invite.wowm#L8):
/// ```text
/// smsg SMSG_CALENDAR_EVENT_INVITE = 0x043A {
///     PackedGuid invitee;
///     Guid event_id;
///     Guid invite_id;
///     Level level;
///     u8 invite_status;
///     CalendarStatusTime time;
///     if (time == PRESENT) {
///         DateTime status_time;
///     }
///     Bool is_sign_up;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_CALENDAR_EVENT_INVITE {
    pub invitee: Guid,
    pub event_id: Guid,
    pub invite_id: Guid,
    pub level: Level,
    pub invite_status: u8,
    pub time: SMSG_CALENDAR_EVENT_INVITE_CalendarStatusTime,
    pub is_sign_up: bool,
}

impl crate::private::Sealed for SMSG_CALENDAR_EVENT_INVITE {}
impl SMSG_CALENDAR_EVENT_INVITE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(21..=33).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // invitee: PackedGuid
        let invitee = crate::util::read_packed_guid(&mut r)?;

        // event_id: Guid
        let event_id = crate::util::read_guid(&mut r)?;

        // invite_id: Guid
        let invite_id = crate::util::read_guid(&mut r)?;

        // level: Level
        let level = Level::new(crate::util::read_u8_le(&mut r)?);

        // invite_status: u8
        let invite_status = crate::util::read_u8_le(&mut r)?;

        // time: CalendarStatusTime
        let time = crate::util::read_u8_le(&mut r)?.try_into()?;

        let time_if = match time {
            CalendarStatusTime::NotPresent => SMSG_CALENDAR_EVENT_INVITE_CalendarStatusTime::NotPresent,
            CalendarStatusTime::Present => {
                // status_time: DateTime
                let status_time = DateTime::try_from(crate::util::read_u32_le(&mut r)?)?;

                SMSG_CALENDAR_EVENT_INVITE_CalendarStatusTime::Present {
                    status_time,
                }
            }
        };

        // is_sign_up: Bool
        let is_sign_up = crate::util::read_bool_u8(&mut r)?;

        Ok(Self {
            invitee,
            event_id,
            invite_id,
            level,
            invite_status,
            time: time_if,
            is_sign_up,
        })
    }

}

impl crate::Message for SMSG_CALENDAR_EVENT_INVITE {
    const OPCODE: u32 = 0x043a;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_CALENDAR_EVENT_INVITE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_CALENDAR_EVENT_INVITE {{").unwrap();
        // Members
        writeln!(s, "    invitee = {};", self.invitee.guid()).unwrap();
        writeln!(s, "    event_id = {};", self.event_id.guid()).unwrap();
        writeln!(s, "    invite_id = {};", self.invite_id.guid()).unwrap();
        writeln!(s, "    level = {};", self.level.as_int()).unwrap();
        writeln!(s, "    invite_status = {};", self.invite_status).unwrap();
        writeln!(s, "    time = {};", CalendarStatusTime::try_from(self.time.as_int()).unwrap().as_test_case_value()).unwrap();
        match &self.time {
            crate::wrath::SMSG_CALENDAR_EVENT_INVITE_CalendarStatusTime::Present {
                status_time,
            } => {
                writeln!(s, "    status_time = {};", status_time.as_int()).unwrap();
            }
            _ => {}
        }

        writeln!(s, "    is_sign_up = {};", if self.is_sign_up { "TRUE" } else { "FALSE" }).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1082_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.invitee), "invitee", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "event_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "invite_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "level", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "invite_status", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "time", "    ");
        match &self.time {
            crate::wrath::SMSG_CALENDAR_EVENT_INVITE_CalendarStatusTime::Present {
                status_time,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 4, "status_time", "    ");
            }
            _ => {}
        }

        crate::util::write_bytes(&mut s, &mut bytes, 1, "is_sign_up", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // invitee: PackedGuid
        crate::util::write_packed_guid(&self.invitee, &mut w)?;

        // event_id: Guid
        w.write_all(&self.event_id.guid().to_le_bytes())?;

        // invite_id: Guid
        w.write_all(&self.invite_id.guid().to_le_bytes())?;

        // level: Level
        w.write_all(&self.level.as_int().to_le_bytes())?;

        // invite_status: u8
        w.write_all(&self.invite_status.to_le_bytes())?;

        // time: CalendarStatusTime
        w.write_all(&(self.time.as_int().to_le_bytes()))?;

        match &self.time {
            SMSG_CALENDAR_EVENT_INVITE_CalendarStatusTime::Present {
                status_time,
            } => {
                // status_time: DateTime
                w.write_all(&status_time.as_int().to_le_bytes())?;

            }
            _ => {}
        }

        // is_sign_up: Bool
        w.write_all(u8::from(self.is_sign_up).to_le_bytes().as_slice())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1082, "SMSG_CALENDAR_EVENT_INVITE", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CALENDAR_EVENT_INVITE {}

impl SMSG_CALENDAR_EVENT_INVITE {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.invitee) // invitee: PackedGuid
        + 8 // event_id: Guid
        + 8 // invite_id: Guid
        + 1 // level: Level
        + 1 // invite_status: u8
        + self.time.size() // time: SMSG_CALENDAR_EVENT_INVITE_CalendarStatusTime
        + 1 // is_sign_up: Bool
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_CALENDAR_EVENT_INVITE_CalendarStatusTime {
    NotPresent,
    Present {
        status_time: DateTime,
    },
}

impl Default for SMSG_CALENDAR_EVENT_INVITE_CalendarStatusTime {
    fn default() -> Self {
        // First enumerator without any fields
        Self::NotPresent
    }
}

impl SMSG_CALENDAR_EVENT_INVITE_CalendarStatusTime {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NotPresent => 0,
            Self::Present { .. } => 1,
        }
    }

}

impl std::fmt::Display for SMSG_CALENDAR_EVENT_INVITE_CalendarStatusTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotPresent => f.write_str("NotPresent"),
            Self::Present{ .. } => f.write_str("Present"),
        }
    }
}

impl SMSG_CALENDAR_EVENT_INVITE_CalendarStatusTime {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::Present {
                ..
            } => {
                1
                + 4 // status_time: DateTime
            }
            _ => 1,
        }
    }
}

