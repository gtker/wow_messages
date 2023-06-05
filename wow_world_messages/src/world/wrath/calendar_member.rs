use std::io::{Read, Write};

use crate::Guid;
use wow_world_base::shared::level_vanilla_tbc_wrath::Level;

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
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // member: PackedGuid
        crate::util::write_packed_guid(&self.member, &mut w)?;

        // level: Level
        w.write_all(&self.level.as_int().to_le_bytes())?;

        Ok(())
    }
}

impl CalendarMember {
    pub(crate) fn read<R: Read>(mut r: R) -> Result<Self, std::io::Error> {
        // member: PackedGuid
        let member = crate::util::read_packed_guid(&mut r)?;

        // level: Level
        let level = Level::new(crate::util::read_u8_le(&mut r)?);

        Ok(Self {
            member,
            level,
        })
    }

}

impl CalendarMember {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.member) // member: PackedGuid
        + 1 // level: Level
    }
}

