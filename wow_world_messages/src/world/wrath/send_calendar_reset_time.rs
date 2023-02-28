use crate::wrath::Map;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/smsg_calendar_send_calendar.wowm:33`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/smsg_calendar_send_calendar.wowm#L33):
/// ```text
/// struct SendCalendarResetTime {
///     Map map;
///     u32 period;
///     u32 time_offset;
/// }
/// ```
pub struct SendCalendarResetTime {
    pub map: Map,
    pub period: u32,
    pub time_offset: u32,
}

impl SendCalendarResetTime {
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // map: Map
        w.write_all(&u32::from(self.map.as_int()).to_le_bytes())?;

        // period: u32
        w.write_all(&self.period.to_le_bytes())?;

        // time_offset: u32
        w.write_all(&self.time_offset.to_le_bytes())?;

        Ok(())
    }
}

impl SendCalendarResetTime {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> std::result::Result<Self, crate::errors::ParseError> {
        // map: Map
        let map: Map = crate::util::read_u32_le(&mut r)?.try_into()?;

        // period: u32
        let period = crate::util::read_u32_le(&mut r)?;

        // time_offset: u32
        let time_offset = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            map,
            period,
            time_offset,
        })
    }

}

