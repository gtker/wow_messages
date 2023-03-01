use crate:: {
    Guid,
};
use crate::wrath:: {
    Level,
};
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/smsg_calendar_filter_guild.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/smsg_calendar_filter_guild.wowm#L1):
/// ```text
/// struct CalendarMember {
///     PackedGuid member;
///     Level level;
/// }
/// ```
pub struct CalendarMember {
    pub member: Guid,
    pub level: Level,
}

impl CalendarMember {
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // member: PackedGuid
        self.member.write_packed_guid_into_vec(&mut w)?;

        // level: Level
        w.write_all(&self.level.as_int().to_le_bytes())?;

        Ok(())
    }
}

impl CalendarMember {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> std::result::Result<Self, std::io::Error> {
        // member: PackedGuid
        let member = Guid::read_packed(&mut r)?;

        // level: Level
        let level = Level::new(crate::util::read_u8_le(&mut r)?);

        Ok(Self {
            member,
            level,
        })
    }

}

impl CalendarMember {
    pub(crate) fn size(&self) -> usize {
        self.member.size() // member: PackedGuid
        + 1 // level: Level
    }
}

