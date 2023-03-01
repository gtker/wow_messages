use crate:: {
    DateTime,
    Guid,
};
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/smsg_calendar_send_calendar.wowm:12`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/smsg_calendar_send_calendar.wowm#L12):
/// ```text
/// struct SendCalendarEvent {
///     Guid event_id;
///     CString title;
///     u32 event_type;
///     DateTime event_time;
///     u32 flags;
///     u32 dungeon_id;
///     PackedGuid creator;
/// }
/// ```
pub struct SendCalendarEvent {
    pub event_id: Guid,
    pub title: String,
    pub event_type: u32,
    pub event_time: DateTime,
    pub flags: u32,
    pub dungeon_id: u32,
    pub creator: Guid,
}

impl SendCalendarEvent {
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // event_id: Guid
        w.write_all(&self.event_id.guid().to_le_bytes())?;

        // title: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.title.as_bytes().iter().rev().next(), Some(&0_u8), "String `title` must not be null-terminated.");
        w.write_all(self.title.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // event_type: u32
        w.write_all(&self.event_type.to_le_bytes())?;

        // event_time: DateTime
        w.write_all(&self.event_time.as_int().to_le_bytes())?;

        // flags: u32
        w.write_all(&self.flags.to_le_bytes())?;

        // dungeon_id: u32
        w.write_all(&self.dungeon_id.to_le_bytes())?;

        // creator: PackedGuid
        self.creator.write_packed_guid_into_vec(&mut w)?;

        Ok(())
    }
}

impl SendCalendarEvent {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> std::result::Result<Self, crate::errors::ParseError> {
        // event_id: Guid
        let event_id = Guid::read(&mut r)?;

        // title: CString
        let title = {
            let title = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(title)?
        };

        // event_type: u32
        let event_type = crate::util::read_u32_le(&mut r)?;

        // event_time: DateTime
        let event_time: DateTime = crate::util::read_u32_le(&mut r)?.try_into()?;

        // flags: u32
        let flags = crate::util::read_u32_le(&mut r)?;

        // dungeon_id: u32
        let dungeon_id = crate::util::read_u32_le(&mut r)?;

        // creator: PackedGuid
        let creator = Guid::read_packed(&mut r)?;

        Ok(Self {
            event_id,
            title,
            event_type,
            event_time,
            flags,
            dungeon_id,
            creator,
        })
    }

}

impl SendCalendarEvent {
    pub(crate) fn size(&self) -> usize {
        8 // event_id: Guid
        + self.title.len() + 1 // title: CString
        + 4 // event_type: u32
        + 4 // event_time: DateTime
        + 4 // flags: u32
        + 4 // dungeon_id: u32
        + self.creator.size() // creator: Guid
    }
}

