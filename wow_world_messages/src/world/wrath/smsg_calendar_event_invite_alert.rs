use std::io::{Read, Write};

use crate::{
    DateTime, Guid,
};

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

impl crate::private::Sealed for SMSG_CALENDAR_EVENT_INVITE_ALERT {}
impl SMSG_CALENDAR_EVENT_INVITE_ALERT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(37..=308).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // event_id: Guid
        let event_id = crate::util::read_guid(&mut r)?;

        // title: CString
        let title = {
            let title = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(title)?
        };

        // event_time: DateTime
        let event_time = DateTime::try_from(crate::util::read_u32_le(&mut r)?)?;

        // flags: u32
        let flags = crate::util::read_u32_le(&mut r)?;

        // event_type: u32
        let event_type = crate::util::read_u32_le(&mut r)?;

        // dungeon_id: u32
        let dungeon_id = crate::util::read_u32_le(&mut r)?;

        // invite_id: Guid
        let invite_id = crate::util::read_guid(&mut r)?;

        // status: u8
        let status = crate::util::read_u8_le(&mut r)?;

        // rank: u8
        let rank = crate::util::read_u8_le(&mut r)?;

        // event_creator: PackedGuid
        let event_creator = crate::util::read_packed_guid(&mut r)?;

        // invite_sender: PackedGuid
        let invite_sender = crate::util::read_packed_guid(&mut r)?;

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

impl crate::Message for SMSG_CALENDAR_EVENT_INVITE_ALERT {
    const OPCODE: u32 = 0x0440;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_CALENDAR_EVENT_INVITE_ALERT {{").unwrap();
        // Members
        writeln!(s, "    event_id = {};", self.event_id.guid()).unwrap();
        writeln!(s, "    title = \"{}\";", self.title).unwrap();
        writeln!(s, "    event_time = {};", self.event_time.as_int()).unwrap();
        writeln!(s, "    flags = {};", self.flags).unwrap();
        writeln!(s, "    event_type = {};", self.event_type).unwrap();
        writeln!(s, "    dungeon_id = {};", self.dungeon_id).unwrap();
        writeln!(s, "    invite_id = {};", self.invite_id.guid()).unwrap();
        writeln!(s, "    status = {};", self.status).unwrap();
        writeln!(s, "    rank = {};", self.rank).unwrap();
        writeln!(s, "    event_creator = {};", self.event_creator.guid()).unwrap();
        writeln!(s, "    invite_sender = {};", self.invite_sender.guid()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1088_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "event_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.title.len() + 1, "title", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "event_time", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "flags", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "event_type", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "dungeon_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "invite_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "status", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "rank", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.event_creator), "event_creator", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.invite_sender), "invite_sender", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // event_id: Guid
        w.write_all(&self.event_id.guid().to_le_bytes())?;

        // title: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.title.as_bytes().iter().next_back(), Some(&0_u8), "String `title` must not be null-terminated.");
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
        crate::util::write_packed_guid(&self.event_creator, &mut w)?;

        // invite_sender: PackedGuid
        crate::util::write_packed_guid(&self.invite_sender, &mut w)?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1088, "SMSG_CALENDAR_EVENT_INVITE_ALERT", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CALENDAR_EVENT_INVITE_ALERT {}

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
        + crate::util::packed_guid_size(&self.event_creator) // event_creator: PackedGuid
        + crate::util::packed_guid_size(&self.invite_sender) // invite_sender: PackedGuid
    }
}

