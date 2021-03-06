use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::map::{Map, map_try_from, map_as_int};
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/smsg_raid_instance_info.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/smsg_raid_instance_info.wowm#L3):
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
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // map: Map
        w.write_all(&(map_as_int(&self.map) as u32).to_le_bytes())?;

        // reset_time: u32
        w.write_all(&self.reset_time.to_le_bytes())?;

        // instance_id: u32
        w.write_all(&self.instance_id.to_le_bytes())?;

        Ok(())
    }
}

impl RaidInfo {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // map: Map
        let map: Map = map_try_from(crate::util::read_u32_le(r)?)?;

        // reset_time: u32
        let reset_time = crate::util::read_u32_le(r)?;

        // instance_id: u32
        let instance_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            map,
            reset_time,
            instance_id,
        })
    }

}

