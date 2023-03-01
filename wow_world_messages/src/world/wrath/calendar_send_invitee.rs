use crate:: {
    DateTime,
    Guid,
};
use crate::wrath:: {
    Level,
};
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/smsg_calendar_send_event.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/smsg_calendar_send_event.wowm#L1):
/// ```text
/// struct CalendarSendInvitee {
///     PackedGuid invitee;
///     Level level;
///     u8 status;
///     u8 rank;
///     u8 guild_member;
///     Guid invite_id;
///     DateTime status_time;
///     CString text;
/// }
/// ```
pub struct CalendarSendInvitee {
    pub invitee: Guid,
    pub level: Level,
    pub status: u8,
    pub rank: u8,
    pub guild_member: u8,
    pub invite_id: Guid,
    pub status_time: DateTime,
    pub text: String,
}

impl CalendarSendInvitee {
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // invitee: PackedGuid
        self.invitee.write_packed_guid_into_vec(&mut w)?;

        // level: Level
        w.write_all(&self.level.as_int().to_le_bytes())?;

        // status: u8
        w.write_all(&self.status.to_le_bytes())?;

        // rank: u8
        w.write_all(&self.rank.to_le_bytes())?;

        // guild_member: u8
        w.write_all(&self.guild_member.to_le_bytes())?;

        // invite_id: Guid
        w.write_all(&self.invite_id.guid().to_le_bytes())?;

        // status_time: DateTime
        w.write_all(&self.status_time.as_int().to_le_bytes())?;

        // text: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.text.as_bytes().iter().rev().next(), Some(&0_u8), "String `text` must not be null-terminated.");
        w.write_all(self.text.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
}

impl CalendarSendInvitee {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> std::result::Result<Self, crate::errors::ParseError> {
        // invitee: PackedGuid
        let invitee = Guid::read_packed(&mut r)?;

        // level: Level
        let level = Level::new(crate::util::read_u8_le(&mut r)?);

        // status: u8
        let status = crate::util::read_u8_le(&mut r)?;

        // rank: u8
        let rank = crate::util::read_u8_le(&mut r)?;

        // guild_member: u8
        let guild_member = crate::util::read_u8_le(&mut r)?;

        // invite_id: Guid
        let invite_id = Guid::read(&mut r)?;

        // status_time: DateTime
        let status_time: DateTime = crate::util::read_u32_le(&mut r)?.try_into()?;

        // text: CString
        let text = {
            let text = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(text)?
        };

        Ok(Self {
            invitee,
            level,
            status,
            rank,
            guild_member,
            invite_id,
            status_time,
            text,
        })
    }

}

impl CalendarSendInvitee {
    pub(crate) fn size(&self) -> usize {
        self.invitee.size() // invitee: Guid
        + 1 // level: Level
        + 1 // status: u8
        + 1 // rank: u8
        + 1 // guild_member: u8
        + 8 // invite_id: Guid
        + 4 // status_time: DateTime
        + self.text.len() + 1 // text: CString
    }
}

