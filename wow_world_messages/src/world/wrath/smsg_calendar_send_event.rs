use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::DateTime;
use crate::wrath::CalendarSendInvitee;
use std::io::{Write, Read};

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

impl crate::Message for SMSG_CALENDAR_SEND_EVENT {
    const OPCODE: u32 = 0x0437;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // send_type: u8
        w.write_all(&self.send_type.to_le_bytes())?;

        // creator: PackedGuid
        self.creator.write_packed_guid_into_vec(w);

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
            i.write_into_vec(w)?;
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(43..=4294967294).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0437, size: body_size as u32 });
        }

        // send_type: u8
        let send_type = crate::util::read_u8_le(r)?;

        // creator: PackedGuid
        let creator = Guid::read_packed(r)?;

        // event_id: Guid
        let event_id = Guid::read(r)?;

        // title: CString
        let title = crate::util::read_c_string_to_vec(r)?;
        let title = String::from_utf8(title)?;

        // description: CString
        let description = crate::util::read_c_string_to_vec(r)?;
        let description = String::from_utf8(description)?;

        // event_type: u8
        let event_type = crate::util::read_u8_le(r)?;

        // repeatable: u8
        let repeatable = crate::util::read_u8_le(r)?;

        // max_invitees: u32
        let max_invitees = crate::util::read_u32_le(r)?;

        // dungeon_id: u32
        let dungeon_id = crate::util::read_u32_le(r)?;

        // flags: u32
        let flags = crate::util::read_u32_le(r)?;

        // event_time: DateTime
        let event_time: DateTime = crate::util::read_u32_le(r)?.try_into()?;
        // time_zone_time: DateTime
        let time_zone_time: DateTime = crate::util::read_u32_le(r)?.try_into()?;
        // guild_id: u32
        let guild_id = crate::util::read_u32_le(r)?;

        // amount_of_invitees: u32
        let amount_of_invitees = crate::util::read_u32_le(r)?;

        // invitees: CalendarSendInvitee[amount_of_invitees]
        let mut invitees = Vec::with_capacity(amount_of_invitees as usize);
        for i in 0..amount_of_invitees {
            invitees.push(CalendarSendInvitee::read(r)?);
        }

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
        + self.creator.size() // creator: Guid
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

