use crate:: {
    Guid,
};
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/smsg_calendar_send_calendar.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/smsg_calendar_send_calendar.wowm#L1):
/// ```text
/// struct SendCalendarInvite {
///     Guid event_id;
///     Guid invite_id;
///     u8 status;
///     u8 rank;
///     Bool is_guild_event;
///     PackedGuid creator;
/// }
/// ```
pub struct SendCalendarInvite {
    pub event_id: Guid,
    pub invite_id: Guid,
    pub status: u8,
    pub rank: u8,
    pub is_guild_event: bool,
    pub creator: Guid,
}

impl SendCalendarInvite {
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // event_id: Guid
        w.write_all(&self.event_id.guid().to_le_bytes())?;

        // invite_id: Guid
        w.write_all(&self.invite_id.guid().to_le_bytes())?;

        // status: u8
        w.write_all(&self.status.to_le_bytes())?;

        // rank: u8
        w.write_all(&self.rank.to_le_bytes())?;

        // is_guild_event: Bool
        w.write_all(u8::from(self.is_guild_event).to_le_bytes().as_slice())?;

        // creator: PackedGuid
        self.creator.write_packed_guid_into_vec(&mut w)?;

        Ok(())
    }
}

impl SendCalendarInvite {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> std::result::Result<Self, std::io::Error> {
        // event_id: Guid
        let event_id = Guid::read(&mut r)?;

        // invite_id: Guid
        let invite_id = Guid::read(&mut r)?;

        // status: u8
        let status = crate::util::read_u8_le(&mut r)?;

        // rank: u8
        let rank = crate::util::read_u8_le(&mut r)?;

        // is_guild_event: Bool
        let is_guild_event = crate::util::read_u8_le(&mut r)? != 0;

        // creator: PackedGuid
        let creator = Guid::read_packed(&mut r)?;

        Ok(Self {
            event_id,
            invite_id,
            status,
            rank,
            is_guild_event,
            creator,
        })
    }

}

impl SendCalendarInvite {
    pub(crate) fn size(&self) -> usize {
        8 // event_id: Guid
        + 8 // invite_id: Guid
        + 1 // status: u8
        + 1 // rank: u8
        + 1 // is_guild_event: Bool
        + self.creator.size() // creator: PackedGuid
    }
}

