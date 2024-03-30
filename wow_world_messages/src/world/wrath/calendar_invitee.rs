use std::io::{Read, Write};

use crate::Guid;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/cmsg_calendar_add_event.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/cmsg_calendar_add_event.wowm#L1):
/// ```text
/// struct CalendarInvitee {
///     PackedGuid guid;
///     u8 status;
///     u8 rank;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CalendarInvitee {
    pub guid: Guid,
    pub status: u8,
    pub rank: u8,
}

impl CalendarInvitee {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        crate::util::write_packed_guid(&self.guid, &mut w)?;

        // status: u8
        w.write_all(&self.status.to_le_bytes())?;

        // rank: u8
        w.write_all(&self.rank.to_le_bytes())?;

        Ok(())
    }
}

impl CalendarInvitee {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, std::io::Error> {
        // guid: PackedGuid
        let guid = crate::util::read_packed_guid(&mut r)?;

        // status: u8
        let status = crate::util::read_u8_le(&mut r)?;

        // rank: u8
        let rank = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            guid,
            status,
            rank,
        })
    }

}

impl CalendarInvitee {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.guid) // guid: PackedGuid
        + 1 // status: u8
        + 1 // rank: u8
    }
}

