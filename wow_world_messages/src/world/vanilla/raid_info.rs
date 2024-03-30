use std::io::{Read, Write};

use crate::vanilla::Map;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/smsg_raid_instance_info.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/smsg_raid_instance_info.wowm#L1):
/// ```text
/// struct RaidInfo {
///     Map map;
///     u32 reset_time;
///     u32 instance_id;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct RaidInfo {
    pub map: Map,
    pub reset_time: u32,
    pub instance_id: u32,
}

impl RaidInfo {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // map: Map
        w.write_all(&(self.map.as_int().to_le_bytes()))?;

        // reset_time: u32
        w.write_all(&self.reset_time.to_le_bytes())?;

        // instance_id: u32
        w.write_all(&self.instance_id.to_le_bytes())?;

        Ok(())
    }
}

impl RaidInfo {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // map: Map
        let map = crate::util::read_u32_le(&mut r)?.try_into()?;

        // reset_time: u32
        let reset_time = crate::util::read_u32_le(&mut r)?;

        // instance_id: u32
        let instance_id = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            map,
            reset_time,
            instance_id,
        })
    }

}

