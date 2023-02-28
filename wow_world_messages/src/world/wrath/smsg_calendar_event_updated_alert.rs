use crate::Guid;
use crate::DateTime;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/smsg_calendar_event_updated_alert.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/smsg_calendar_event_updated_alert.wowm#L1):
/// ```text
/// smsg SMSG_CALENDAR_EVENT_UPDATED_ALERT = 0x0444 {
///     Bool show_alert;
///     Guid event_id;
///     DateTime old_event_time;
///     u32 flags;
///     DateTime new_event_time;
///     u8 event_type;
///     u32 dungeon_id;
///     CString title;
///     CString description;
///     u8 repeatable;
///     u32 max_invitees;
///     DateTime unknown_time;
/// }
/// ```
pub struct SMSG_CALENDAR_EVENT_UPDATED_ALERT {
    pub show_alert: bool,
    pub event_id: Guid,
    pub old_event_time: DateTime,
    pub flags: u32,
    pub new_event_time: DateTime,
    pub event_type: u8,
    pub dungeon_id: u32,
    pub title: String,
    pub description: String,
    pub repeatable: u8,
    pub max_invitees: u32,
    pub unknown_time: DateTime,
}

impl crate::Message for SMSG_CALENDAR_EVENT_UPDATED_ALERT {
    const OPCODE: u32 = 0x0444;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // show_alert: Bool
        w.write_all(u8::from(self.show_alert).to_le_bytes().as_slice())?;

        // event_id: Guid
        w.write_all(&self.event_id.guid().to_le_bytes())?;

        // old_event_time: DateTime
        w.write_all(&self.old_event_time.as_int().to_le_bytes())?;

        // flags: u32
        w.write_all(&self.flags.to_le_bytes())?;

        // new_event_time: DateTime
        w.write_all(&self.new_event_time.as_int().to_le_bytes())?;

        // event_type: u8
        w.write_all(&self.event_type.to_le_bytes())?;

        // dungeon_id: u32
        w.write_all(&self.dungeon_id.to_le_bytes())?;

        // title: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.title.as_bytes().iter().rev().next(), Some(&0_u8), "String `title` must not be null-terminated.");
        w.write_all(self.title.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // description: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.description.as_bytes().iter().rev().next(), Some(&0_u8), "String `description` must not be null-terminated.");
        w.write_all(self.description.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // repeatable: u8
        w.write_all(&self.repeatable.to_le_bytes())?;

        // max_invitees: u32
        w.write_all(&self.max_invitees.to_le_bytes())?;

        // unknown_time: DateTime
        w.write_all(&self.unknown_time.as_int().to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(37..=547).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0444, size: body_size as u32 });
        }

        // show_alert: Bool
        let show_alert = crate::util::read_u8_le(&mut r)? != 0;

        // event_id: Guid
        let event_id = Guid::read(&mut r)?;

        // old_event_time: DateTime
        let old_event_time: DateTime = crate::util::read_u32_le(&mut r)?.try_into()?;

        // flags: u32
        let flags = crate::util::read_u32_le(&mut r)?;

        // new_event_time: DateTime
        let new_event_time: DateTime = crate::util::read_u32_le(&mut r)?.try_into()?;

        // event_type: u8
        let event_type = crate::util::read_u8_le(&mut r)?;

        // dungeon_id: u32
        let dungeon_id = crate::util::read_u32_le(&mut r)?;

        // title: CString
        let title = {
            let title = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(title)?
        };

        // description: CString
        let description = {
            let description = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(description)?
        };

        // repeatable: u8
        let repeatable = crate::util::read_u8_le(&mut r)?;

        // max_invitees: u32
        let max_invitees = crate::util::read_u32_le(&mut r)?;

        // unknown_time: DateTime
        let unknown_time: DateTime = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            show_alert,
            event_id,
            old_event_time,
            flags,
            new_event_time,
            event_type,
            dungeon_id,
            title,
            description,
            repeatable,
            max_invitees,
            unknown_time,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CALENDAR_EVENT_UPDATED_ALERT {}

impl SMSG_CALENDAR_EVENT_UPDATED_ALERT {
    pub(crate) fn size(&self) -> usize {
        1 // show_alert: Bool
        + 8 // event_id: Guid
        + 4 // old_event_time: DateTime
        + 4 // flags: u32
        + 4 // new_event_time: DateTime
        + 1 // event_type: u8
        + 4 // dungeon_id: u32
        + self.title.len() + 1 // title: CString
        + self.description.len() + 1 // description: CString
        + 1 // repeatable: u8
        + 4 // max_invitees: u32
        + 4 // unknown_time: DateTime
    }
}

