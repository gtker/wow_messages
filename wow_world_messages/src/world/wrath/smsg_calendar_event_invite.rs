use std::io::{Read, Write};

use crate::{
    DateTime, Guid,
};
use crate::wrath::CalendarStatusTime;
use wow_world_base::shared::level_vanilla_tbc_wrath::Level;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
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
impl crate::Message for SMSG_CALENDAR_EVENT_INVITE {
    const OPCODE: u32 = 0x043a;

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
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(22..=33).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x043A, size: body_size });
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
        let is_sign_up = crate::util::read_u8_le(&mut r)? != 0;

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

