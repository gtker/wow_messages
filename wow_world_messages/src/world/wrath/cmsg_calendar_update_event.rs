use std::io::{Read, Write};

use crate::{
    DateTime, Guid,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/cmsg_calendar_update_event.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/cmsg_calendar_update_event.wowm#L1):
/// ```text
/// cmsg CMSG_CALENDAR_UPDATE_EVENT = 0x042E {
///     Guid event;
///     Guid invite_id;
///     CString title;
///     CString description;
///     u8 event_type;
///     Bool repeatable;
///     u32 maximum_invites;
///     u32 dungeon_id;
///     DateTime event_time;
///     DateTime time_zone_time;
///     u32 flags;
/// }
/// ```
pub struct CMSG_CALENDAR_UPDATE_EVENT {
    pub event: Guid,
    pub invite_id: Guid,
    pub title: String,
    pub description: String,
    pub event_type: u8,
    pub repeatable: bool,
    pub maximum_invites: u32,
    pub dungeon_id: u32,
    pub event_time: DateTime,
    pub time_zone_time: DateTime,
    pub flags: u32,
}

impl crate::private::Sealed for CMSG_CALENDAR_UPDATE_EVENT {}
impl crate::Message for CMSG_CALENDAR_UPDATE_EVENT {
    const OPCODE: u32 = 0x042e;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // event: Guid
        w.write_all(&self.event.guid().to_le_bytes())?;

        // invite_id: Guid
        w.write_all(&self.invite_id.guid().to_le_bytes())?;

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

        // event_type: u8
        w.write_all(&self.event_type.to_le_bytes())?;

        // repeatable: Bool
        w.write_all(u8::from(self.repeatable).to_le_bytes().as_slice())?;

        // maximum_invites: u32
        w.write_all(&self.maximum_invites.to_le_bytes())?;

        // dungeon_id: u32
        w.write_all(&self.dungeon_id.to_le_bytes())?;

        // event_time: DateTime
        w.write_all(&self.event_time.as_int().to_le_bytes())?;

        // time_zone_time: DateTime
        w.write_all(&self.time_zone_time.as_int().to_le_bytes())?;

        // flags: u32
        w.write_all(&self.flags.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(40..=550).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x042E, size: body_size });
        }

        // event: Guid
        let event = Guid::read(&mut r)?;

        // invite_id: Guid
        let invite_id = Guid::read(&mut r)?;

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

        // event_type: u8
        let event_type = crate::util::read_u8_le(&mut r)?;

        // repeatable: Bool
        let repeatable = crate::util::read_u8_le(&mut r)? != 0;

        // maximum_invites: u32
        let maximum_invites = crate::util::read_u32_le(&mut r)?;

        // dungeon_id: u32
        let dungeon_id = crate::util::read_u32_le(&mut r)?;

        // event_time: DateTime
        let event_time: DateTime = crate::util::read_u32_le(&mut r)?.try_into()?;

        // time_zone_time: DateTime
        let time_zone_time: DateTime = crate::util::read_u32_le(&mut r)?.try_into()?;

        // flags: u32
        let flags = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            event,
            invite_id,
            title,
            description,
            event_type,
            repeatable,
            maximum_invites,
            dungeon_id,
            event_time,
            time_zone_time,
            flags,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_CALENDAR_UPDATE_EVENT {}

impl CMSG_CALENDAR_UPDATE_EVENT {
    pub(crate) fn size(&self) -> usize {
        8 // event: Guid
        + 8 // invite_id: Guid
        + self.title.len() + 1 // title: CString
        + self.description.len() + 1 // description: CString
        + 1 // event_type: u8
        + 1 // repeatable: Bool
        + 4 // maximum_invites: u32
        + 4 // dungeon_id: u32
        + 4 // event_time: DateTime
        + 4 // time_zone_time: DateTime
        + 4 // flags: u32
    }
}

