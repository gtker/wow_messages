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

#[cfg(feature = "print-testcase")]
impl CMSG_CALENDAR_UPDATE_EVENT {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_CALENDAR_UPDATE_EVENT {{").unwrap();
        // Members
        writeln!(s, "    event = {};", self.event.guid()).unwrap();
        writeln!(s, "    invite_id = {};", self.invite_id.guid()).unwrap();
        writeln!(s, "    title = \"{}\";", self.title).unwrap();
        writeln!(s, "    description = \"{}\";", self.description).unwrap();
        writeln!(s, "    event_type = {};", self.event_type).unwrap();
        writeln!(s, "    repeatable = {};", if self.repeatable { "TRUE" } else { "FALSE" }).unwrap();
        writeln!(s, "    maximum_invites = {};", self.maximum_invites).unwrap();
        writeln!(s, "    dungeon_id = {};", self.dungeon_id).unwrap();
        writeln!(s, "    event_time = {};", self.event_time.as_int()).unwrap();
        writeln!(s, "    time_zone_time = {};", self.time_zone_time.as_int()).unwrap();
        writeln!(s, "    flags = {};", self.flags).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1070_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "event", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "invite_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.title.len() + 1, "title", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.description.len() + 1, "description", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "event_type", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "repeatable", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "maximum_invites", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "dungeon_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "event_time", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_zone_time", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "flags", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for CMSG_CALENDAR_UPDATE_EVENT {}
impl crate::Message for CMSG_CALENDAR_UPDATE_EVENT {
    const OPCODE: u32 = 0x042e;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        CMSG_CALENDAR_UPDATE_EVENT::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
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
        let event = crate::util::read_guid(&mut r)?;

        // invite_id: Guid
        let invite_id = crate::util::read_guid(&mut r)?;

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
        let event_time = DateTime::try_from(crate::util::read_u32_le(&mut r)?)?;

        // time_zone_time: DateTime
        let time_zone_time = DateTime::try_from(crate::util::read_u32_le(&mut r)?)?;

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

