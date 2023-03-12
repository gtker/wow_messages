use std::io::{Read, Write};

use crate::vanilla::Map;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/smsg_raid_instance_info.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/smsg_raid_instance_info.wowm#L1):
/// ```text
/// struct RaidInfo {
///     Map map;
///     u32 reset_time;
///     u32 instance_id;
/// }
/// ```
pub struct RaidInfo {
    pub map: Map,
    pub reset_time: u32,
    pub instance_id: u32,
}

impl RaidInfo {
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // map: Map
        w.write_all(&u32::from(self.map.as_int()).to_le_bytes())?;

        // reset_time: u32
        w.write_all(&self.reset_time.to_le_bytes())?;

        // instance_id: u32
        w.write_all(&self.instance_id.to_le_bytes())?;

        Ok(())
    }
}

impl RaidInfo {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> std::result::Result<Self, crate::errors::ParseError> {
        // map: Map
        let map: Map = crate::util::read_u32_le(&mut r)?.try_into()?;

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

