use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/smsg_calendar_filter_guild.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/smsg_calendar_filter_guild.wowm#L1):
/// ```text
/// struct CalendarMember {
///     PackedGuid member;
///     u8 level;
/// }
/// ```
pub struct CalendarMember {
    pub member: Guid,
    pub level: u8,
}

impl CalendarMember {
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // member: PackedGuid
        self.member.write_packed_guid_into_vec(&mut w)?;

        // level: u8
        w.write_all(&self.level.to_le_bytes())?;

        Ok(())
    }
}

impl CalendarMember {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> std::result::Result<Self, std::io::Error> {
        // member: PackedGuid
        let member = Guid::read_packed(&mut r)?;

        // level: u8
        let level = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            member,
            level,
        })
    }

}

impl CalendarMember {
    pub(crate) fn size(&self) -> usize {
        self.member.size() // member: Guid
        + 1 // level: u8
    }
}

