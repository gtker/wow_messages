use std::io::{Read, Write};

use crate::DateTime;
use crate::wrath::Map;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/smsg_calendar_raid_lockout_updated.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/smsg_calendar_raid_lockout_updated.wowm#L1):
/// ```text
/// smsg SMSG_CALENDAR_RAID_LOCKOUT_UPDATED = 0x0471 {
///     DateTime current_time;
///     Map map;
///     u32 difficulty;
///     Seconds old_time_to_update;
///     Seconds new_time_to_update;
/// }
/// ```
pub struct SMSG_CALENDAR_RAID_LOCKOUT_UPDATED {
    pub current_time: DateTime,
    pub map: Map,
    pub difficulty: u32,
    pub old_time_to_update: Duration,
    pub new_time_to_update: Duration,
}

impl crate::private::Sealed for SMSG_CALENDAR_RAID_LOCKOUT_UPDATED {}
impl crate::Message for SMSG_CALENDAR_RAID_LOCKOUT_UPDATED {
    const OPCODE: u32 = 0x0471;

    fn size_without_header(&self) -> u32 {
        20
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // current_time: DateTime
        w.write_all(&self.current_time.as_int().to_le_bytes())?;

        // map: Map
        w.write_all(&(self.map.as_int().to_le_bytes()))?;

        // difficulty: u32
        w.write_all(&self.difficulty.to_le_bytes())?;

        // old_time_to_update: Seconds
        w.write_all((self.old_time_to_update.as_secs() as u32).to_le_bytes().as_slice())?;

        // new_time_to_update: Seconds
        w.write_all((self.new_time_to_update.as_secs() as u32).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 20 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0471, size: body_size });
        }

        // current_time: DateTime
        let current_time = DateTime::try_from(crate::util::read_u32_le(&mut r)?)?;

        // map: Map
        let map = crate::util::read_u32_le(&mut r)?.try_into()?;

        // difficulty: u32
        let difficulty = crate::util::read_u32_le(&mut r)?;

        // old_time_to_update: Seconds
        let old_time_to_update = Duration::from_secs(crate::util::read_u32_le(&mut r)?.into());

        // new_time_to_update: Seconds
        let new_time_to_update = Duration::from_secs(crate::util::read_u32_le(&mut r)?.into());

        Ok(Self {
            current_time,
            map,
            difficulty,
            old_time_to_update,
            new_time_to_update,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CALENDAR_RAID_LOCKOUT_UPDATED {}

