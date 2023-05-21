use std::io::{Read, Write};

use crate::{
    DateTime, Guid,
};
use crate::wrath::CalendarInvitee;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/cmsg_calendar_add_event.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/cmsg_calendar_add_event.wowm#L9):
/// ```text
/// cmsg CMSG_CALENDAR_ADD_EVENT = 0x042D {
///     CString title;
///     CString description;
///     u8 event_type;
///     Bool repeatable;
///     u32 maximum_invites;
///     u32 dungeon_id;
///     DateTime event_time;
///     DateTime time_zone_time;
///     u32 flags;
///     u32 amount_of_invitees;
///     CalendarInvitee[amount_of_invitees] invitees;
/// }
/// ```
pub struct CMSG_CALENDAR_ADD_EVENT {
    pub title: String,
    pub description: String,
    pub event_type: u8,
    pub repeatable: bool,
    pub maximum_invites: u32,
    pub dungeon_id: u32,
    pub event_time: DateTime,
    pub time_zone_time: DateTime,
    pub flags: u32,
    pub invitees: Vec<CalendarInvitee>,
}

impl crate::private::Sealed for CMSG_CALENDAR_ADD_EVENT {}
impl CMSG_CALENDAR_ADD_EVENT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(28..=10240).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x042D, size: body_size });
        }

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

        // amount_of_invitees: u32
        let amount_of_invitees = crate::util::read_u32_le(&mut r)?;

        // invitees: CalendarInvitee[amount_of_invitees]
        let invitees = {
            let mut invitees = Vec::with_capacity(amount_of_invitees as usize);
            for _ in 0..amount_of_invitees {
                invitees.push(CalendarInvitee::read(&mut r)?);
            }
            invitees
        };

        Ok(Self {
            title,
            description,
            event_type,
            repeatable,
            maximum_invites,
            dungeon_id,
            event_time,
            time_zone_time,
            flags,
            invitees,
        })
    }

}

impl crate::Message for CMSG_CALENDAR_ADD_EVENT {
    const OPCODE: u32 = 0x042d;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_CALENDAR_ADD_EVENT {{").unwrap();
        // Members
        writeln!(s, "    title = \"{}\";", self.title).unwrap();
        writeln!(s, "    description = \"{}\";", self.description).unwrap();
        writeln!(s, "    event_type = {};", self.event_type).unwrap();
        writeln!(s, "    repeatable = {};", if self.repeatable { "TRUE" } else { "FALSE" }).unwrap();
        writeln!(s, "    maximum_invites = {};", self.maximum_invites).unwrap();
        writeln!(s, "    dungeon_id = {};", self.dungeon_id).unwrap();
        writeln!(s, "    event_time = {};", self.event_time.as_int()).unwrap();
        writeln!(s, "    time_zone_time = {};", self.time_zone_time.as_int()).unwrap();
        writeln!(s, "    flags = {};", self.flags).unwrap();
        writeln!(s, "    amount_of_invitees = {};", self.invitees.len()).unwrap();
        write!(s, "    invitees = [").unwrap();
        for v in self.invitees.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        guid = {};", v.guid.guid()).unwrap();
            writeln!(s, "        status = {};", v.status).unwrap();
            writeln!(s, "        rank = {};", v.rank).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1069_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, self.title.len() + 1, "title", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.description.len() + 1, "description", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "event_type", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "repeatable", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "maximum_invites", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "dungeon_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "event_time", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_zone_time", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "flags", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_invitees", "    ");
        if !self.invitees.is_empty() {
            writeln!(s, "    /* invitees: CalendarInvitee[amount_of_invitees] start */").unwrap();
            for (i, v) in self.invitees.iter().enumerate() {
                writeln!(s, "    /* invitees: CalendarInvitee[amount_of_invitees] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&v.guid), "guid", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "rank", "        ");
                writeln!(s, "    /* invitees: CalendarInvitee[amount_of_invitees] {i} end */").unwrap();
            }
            writeln!(s, "    /* invitees: CalendarInvitee[amount_of_invitees] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
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

        // amount_of_invitees: u32
        w.write_all(&(self.invitees.len() as u32).to_le_bytes())?;

        // invitees: CalendarInvitee[amount_of_invitees]
        for i in self.invitees.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_CALENDAR_ADD_EVENT {}

impl CMSG_CALENDAR_ADD_EVENT {
    pub(crate) fn size(&self) -> usize {
        self.title.len() + 1 // title: CString
        + self.description.len() + 1 // description: CString
        + 1 // event_type: u8
        + 1 // repeatable: Bool
        + 4 // maximum_invites: u32
        + 4 // dungeon_id: u32
        + 4 // event_time: DateTime
        + 4 // time_zone_time: DateTime
        + 4 // flags: u32
        + 4 // amount_of_invitees: u32
        + self.invitees.iter().fold(0, |acc, x| acc + x.size()) // invitees: CalendarInvitee[amount_of_invitees]
    }
}

