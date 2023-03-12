use std::io::{Read, Write};
use crate::Guid;
use crate::wrath::Map;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/smsg_calendar_send_calendar.wowm:24`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/smsg_calendar_send_calendar.wowm#L24):
/// ```text
/// struct SendCalendarInstance {
///     Map map;
///     u32 difficulty;
///     u32 reset_time;
///     Guid instance_id;
/// }
/// ```
pub struct SendCalendarInstance {
    pub map: Map,
    pub difficulty: u32,
    pub reset_time: u32,
    pub instance_id: Guid,
}

impl SendCalendarInstance {
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // map: Map
        w.write_all(&u32::from(self.map.as_int()).to_le_bytes())?;

        // difficulty: u32
        w.write_all(&self.difficulty.to_le_bytes())?;

        // reset_time: u32
        w.write_all(&self.reset_time.to_le_bytes())?;

        // instance_id: Guid
        w.write_all(&self.instance_id.guid().to_le_bytes())?;

        Ok(())
    }
}

impl SendCalendarInstance {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> std::result::Result<Self, crate::errors::ParseError> {
        // map: Map
        let map: Map = crate::util::read_u32_le(&mut r)?.try_into()?;

        // difficulty: u32
        let difficulty = crate::util::read_u32_le(&mut r)?;

        // reset_time: u32
        let reset_time = crate::util::read_u32_le(&mut r)?;

        // instance_id: Guid
        let instance_id = Guid::read(&mut r)?;

        Ok(Self {
            map,
            difficulty,
            reset_time,
            instance_id,
        })
    }

}

