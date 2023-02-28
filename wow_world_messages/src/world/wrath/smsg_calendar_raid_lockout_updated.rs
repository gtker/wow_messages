use crate::DateTime;
use crate::wrath::Map;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/smsg_calendar_raid_lockout_updated.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/smsg_calendar_raid_lockout_updated.wowm#L1):
/// ```text
/// smsg SMSG_CALENDAR_RAID_LOCKOUT_UPDATED = 0x0471 {
///     DateTime current_time;
///     Map map;
///     u32 difficulty;
///     u32 old_time_to_update_in_seconds;
///     u32 new_time_to_update_in_seconds;
/// }
/// ```
pub struct SMSG_CALENDAR_RAID_LOCKOUT_UPDATED {
    pub current_time: DateTime,
    pub map: Map,
    pub difficulty: u32,
    pub old_time_to_update_in_seconds: u32,
    pub new_time_to_update_in_seconds: u32,
}

impl crate::Message for SMSG_CALENDAR_RAID_LOCKOUT_UPDATED {
    const OPCODE: u32 = 0x0471;

    fn size_without_header(&self) -> u32 {
        20
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // current_time: DateTime
        w.write_all(&self.current_time.as_int().to_le_bytes())?;

        // map: Map
        w.write_all(&u32::from(self.map.as_int()).to_le_bytes())?;

        // difficulty: u32
        w.write_all(&self.difficulty.to_le_bytes())?;

        // old_time_to_update_in_seconds: u32
        w.write_all(&self.old_time_to_update_in_seconds.to_le_bytes())?;

        // new_time_to_update_in_seconds: u32
        w.write_all(&self.new_time_to_update_in_seconds.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 20 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0471, size: body_size as u32 });
        }

        // current_time: DateTime
        let current_time: DateTime = crate::util::read_u32_le(r)?.try_into()?;

        // map: Map
        let map: Map = crate::util::read_u32_le(r)?.try_into()?;

        // difficulty: u32
        let difficulty = crate::util::read_u32_le(r)?;

        // old_time_to_update_in_seconds: u32
        let old_time_to_update_in_seconds = crate::util::read_u32_le(r)?;

        // new_time_to_update_in_seconds: u32
        let new_time_to_update_in_seconds = crate::util::read_u32_le(r)?;

        Ok(Self {
            current_time,
            map,
            difficulty,
            old_time_to_update_in_seconds,
            new_time_to_update_in_seconds,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CALENDAR_RAID_LOCKOUT_UPDATED {}

