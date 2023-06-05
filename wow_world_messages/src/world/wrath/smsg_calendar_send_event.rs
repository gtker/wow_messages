use std::io::{Read, Write};

use crate::{
    DateTime, Guid,
};
use crate::wrath::CalendarSendInvitee;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/smsg_calendar_send_event.wowm:14`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/smsg_calendar_send_event.wowm#L14):
/// ```text
/// smsg SMSG_CALENDAR_SEND_EVENT = 0x0437 {
///     u8 send_type;
///     PackedGuid creator;
///     Guid event_id;
///     CString title;
///     CString description;
///     u8 event_type;
///     u8 repeatable;
///     u32 max_invitees;
///     u32 dungeon_id;
///     u32 flags;
///     DateTime event_time;
///     DateTime time_zone_time;
///     u32 guild_id;
///     u32 amount_of_invitees;
///     CalendarSendInvitee[amount_of_invitees] invitees;
/// }
/// ```
pub struct SMSG_CALENDAR_SEND_EVENT {
    pub send_type: u8,
    pub creator: Guid,
    pub event_id: Guid,
    pub title: String,
    pub description: String,
    pub event_type: u8,
    pub repeatable: u8,
    pub max_invitees: u32,
    pub dungeon_id: u32,
    pub flags: u32,
    pub event_time: DateTime,
    pub time_zone_time: DateTime,
    pub guild_id: u32,
    pub invitees: Vec<CalendarSendInvitee>,
}

#[cfg(feature = "print-testcase")]
impl SMSG_CALENDAR_SEND_EVENT {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_CALENDAR_SEND_EVENT {{").unwrap();
        // Members
        writeln!(s, "    send_type = {};", self.send_type).unwrap();
        writeln!(s, "    creator = {};", self.creator.guid()).unwrap();
        writeln!(s, "    event_id = {};", self.event_id.guid()).unwrap();
        writeln!(s, "    title = \"{}\";", self.title).unwrap();
        writeln!(s, "    description = \"{}\";", self.description).unwrap();
        writeln!(s, "    event_type = {};", self.event_type).unwrap();
        writeln!(s, "    repeatable = {};", self.repeatable).unwrap();
        writeln!(s, "    max_invitees = {};", self.max_invitees).unwrap();
        writeln!(s, "    dungeon_id = {};", self.dungeon_id).unwrap();
        writeln!(s, "    flags = {};", self.flags).unwrap();
        writeln!(s, "    event_time = {};", self.event_time.as_int()).unwrap();
        writeln!(s, "    time_zone_time = {};", self.time_zone_time.as_int()).unwrap();
        writeln!(s, "    guild_id = {};", self.guild_id).unwrap();
        writeln!(s, "    amount_of_invitees = {};", self.invitees.len()).unwrap();
        write!(s, "    invitees = [").unwrap();
        for v in self.invitees.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        invitee = {};", v.invitee.guid()).unwrap();
            writeln!(s, "        level = {};", v.level.as_int()).unwrap();
            writeln!(s, "        status = {};", v.status).unwrap();
            writeln!(s, "        rank = {};", v.rank).unwrap();
            writeln!(s, "        guild_member = {};", v.guild_member).unwrap();
            writeln!(s, "        invite_id = {};", v.invite_id.guid()).unwrap();
            writeln!(s, "        status_time = {};", v.status_time.as_int()).unwrap();
            writeln!(s, "        text = \"{}\";", v.text).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1079_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "send_type", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.creator), "creator", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "event_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.title.len() + 1, "title", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.description.len() + 1, "description", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "event_type", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "repeatable", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "max_invitees", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "dungeon_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "flags", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "event_time", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_zone_time", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "guild_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_invitees", "    ");
        if !self.invitees.is_empty() {
            writeln!(s, "    /* invitees: CalendarSendInvitee[amount_of_invitees] start */").unwrap();
            for (i, v) in self.invitees.iter().enumerate() {
                writeln!(s, "    /* invitees: CalendarSendInvitee[amount_of_invitees] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&v.invitee), "invitee", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "level", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "rank", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "guild_member", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "invite_id", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "status_time", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, v.text.len() + 1, "text", "        ");
                writeln!(s, "    /* invitees: CalendarSendInvitee[amount_of_invitees] {i} end */").unwrap();
            }
            writeln!(s, "    /* invitees: CalendarSendInvitee[amount_of_invitees] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_CALENDAR_SEND_EVENT {}
impl crate::Message for SMSG_CALENDAR_SEND_EVENT {
    const OPCODE: u32 = 0x0437;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_CALENDAR_SEND_EVENT::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // send_type: u8
        w.write_all(&self.send_type.to_le_bytes())?;

        // creator: PackedGuid
        crate::util::write_packed_guid(&self.creator, &mut w)?;

        // event_id: Guid
        w.write_all(&self.event_id.guid().to_le_bytes())?;

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

        // repeatable: u8
        w.write_all(&self.repeatable.to_le_bytes())?;

        // max_invitees: u32
        w.write_all(&self.max_invitees.to_le_bytes())?;

        // dungeon_id: u32
        w.write_all(&self.dungeon_id.to_le_bytes())?;

        // flags: u32
        w.write_all(&self.flags.to_le_bytes())?;

        // event_time: DateTime
        w.write_all(&self.event_time.as_int().to_le_bytes())?;

        // time_zone_time: DateTime
        w.write_all(&self.time_zone_time.as_int().to_le_bytes())?;

        // guild_id: u32
        w.write_all(&self.guild_id.to_le_bytes())?;

        // amount_of_invitees: u32
        w.write_all(&(self.invitees.len() as u32).to_le_bytes())?;

        // invitees: CalendarSendInvitee[amount_of_invitees]
        for i in self.invitees.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(43..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0437, size: body_size });
        }

        // send_type: u8
        let send_type = crate::util::read_u8_le(&mut r)?;

        // creator: PackedGuid
        let creator = crate::util::read_packed_guid(&mut r)?;

        // event_id: Guid
        let event_id = crate::util::read_guid(&mut r)?;

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

        // repeatable: u8
        let repeatable = crate::util::read_u8_le(&mut r)?;

        // max_invitees: u32
        let max_invitees = crate::util::read_u32_le(&mut r)?;

        // dungeon_id: u32
        let dungeon_id = crate::util::read_u32_le(&mut r)?;

        // flags: u32
        let flags = crate::util::read_u32_le(&mut r)?;

        // event_time: DateTime
        let event_time = DateTime::try_from(crate::util::read_u32_le(&mut r)?)?;

        // time_zone_time: DateTime
        let time_zone_time = DateTime::try_from(crate::util::read_u32_le(&mut r)?)?;

        // guild_id: u32
        let guild_id = crate::util::read_u32_le(&mut r)?;

        // amount_of_invitees: u32
        let amount_of_invitees = crate::util::read_u32_le(&mut r)?;

        // invitees: CalendarSendInvitee[amount_of_invitees]
        let invitees = {
            let mut invitees = Vec::with_capacity(amount_of_invitees as usize);
            for _ in 0..amount_of_invitees {
                invitees.push(CalendarSendInvitee::read(&mut r)?);
            }
            invitees
        };

        Ok(Self {
            send_type,
            creator,
            event_id,
            title,
            description,
            event_type,
            repeatable,
            max_invitees,
            dungeon_id,
            flags,
            event_time,
            time_zone_time,
            guild_id,
            invitees,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CALENDAR_SEND_EVENT {}

impl SMSG_CALENDAR_SEND_EVENT {
    pub(crate) fn size(&self) -> usize {
        1 // send_type: u8
        + crate::util::packed_guid_size(&self.creator) // creator: PackedGuid
        + 8 // event_id: Guid
        + self.title.len() + 1 // title: CString
        + self.description.len() + 1 // description: CString
        + 1 // event_type: u8
        + 1 // repeatable: u8
        + 4 // max_invitees: u32
        + 4 // dungeon_id: u32
        + 4 // flags: u32
        + 4 // event_time: DateTime
        + 4 // time_zone_time: DateTime
        + 4 // guild_id: u32
        + 4 // amount_of_invitees: u32
        + self.invitees.iter().fold(0, |acc, x| acc + x.size()) // invitees: CalendarSendInvitee[amount_of_invitees]
    }
}

