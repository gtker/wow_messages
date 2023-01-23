use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::DateTime;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/smsg_calendar_event_invite_alert.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/smsg_calendar_event_invite_alert.wowm#L1):
/// ```text
/// smsg SMSG_CALENDAR_EVENT_INVITE_ALERT = 0x0440 {
///     Guid event_id;
///     CString title;
///     DateTime event_time;
///     u32 flags;
///     u32 event_type;
///     u32 dungeon_id;
///     Guid invite_id;
///     u8 status;
///     u8 rank;
///     PackedGuid event_creator;
///     PackedGuid invite_sender;
/// }
/// ```
pub struct SMSG_CALENDAR_EVENT_INVITE_ALERT {
    pub event_id: Guid,
    pub title: String,
    pub event_time: DateTime,
    pub flags: u32,
    pub event_type: u32,
    pub dungeon_id: u32,
    pub invite_id: Guid,
    pub status: u8,
    pub rank: u8,
    pub event_creator: Guid,
    pub invite_sender: Guid,
}

impl crate::Message for SMSG_CALENDAR_EVENT_INVITE_ALERT {
    const OPCODE: u32 = 0x0440;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // event_id: Guid
        w.write_all(&self.event_id.guid().to_le_bytes())?;

        // title: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.title.as_bytes().iter().rev().next(), Some(&0_u8), "String `title` must not be null-terminated.");
        w.write_all(self.title.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // event_time: DateTime
        w.write_all(&self.event_time.as_int().to_le_bytes())?;

        // flags: u32
        w.write_all(&self.flags.to_le_bytes())?;

        // event_type: u32
        w.write_all(&self.event_type.to_le_bytes())?;

        // dungeon_id: u32
        w.write_all(&self.dungeon_id.to_le_bytes())?;

        // invite_id: Guid
        w.write_all(&self.invite_id.guid().to_le_bytes())?;

        // status: u8
        w.write_all(&self.status.to_le_bytes())?;

        // rank: u8
        w.write_all(&self.rank.to_le_bytes())?;

        // event_creator: PackedGuid
        self.event_creator.write_packed_guid_into_vec(w);

        // invite_sender: PackedGuid
        self.invite_sender.write_packed_guid_into_vec(w);

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(39..=308).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0440, size: body_size as u32 });
        }

        // event_id: Guid
        let event_id = Guid::read(r)?;

        // title: CString
        let title = crate::util::read_c_string_to_vec(r)?;
        let title = String::from_utf8(title)?;

        // event_time: DateTime
        let event_time: DateTime = crate::util::read_u32_le(r)?.try_into()?;
        // flags: u32
        let flags = crate::util::read_u32_le(r)?;

        // event_type: u32
        let event_type = crate::util::read_u32_le(r)?;

        // dungeon_id: u32
        let dungeon_id = crate::util::read_u32_le(r)?;

        // invite_id: Guid
        let invite_id = Guid::read(r)?;

        // status: u8
        let status = crate::util::read_u8_le(r)?;

        // rank: u8
        let rank = crate::util::read_u8_le(r)?;

        // event_creator: PackedGuid
        let event_creator = Guid::read_packed(r)?;

        // invite_sender: PackedGuid
        let invite_sender = Guid::read_packed(r)?;

        Ok(Self {
            event_id,
            title,
            event_time,
            flags,
            event_type,
            dungeon_id,
            invite_id,
            status,
            rank,
            event_creator,
            invite_sender,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_CALENDAR_EVENT_INVITE_ALERT {}

impl SMSG_CALENDAR_EVENT_INVITE_ALERT {
    pub(crate) fn size(&self) -> usize {
        8 // event_id: Guid
        + self.title.len() + 1 // title: CString
        + 4 // event_time: DateTime
        + 4 // flags: u32
        + 4 // event_type: u32
        + 4 // dungeon_id: u32
        + 8 // invite_id: Guid
        + 1 // status: u8
        + 1 // rank: u8
        + self.event_creator.size() // event_creator: Guid
        + self.invite_sender.size() // invite_sender: Guid
    }
}

