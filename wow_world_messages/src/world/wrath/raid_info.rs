use std::io::{Read, Write};

use crate::wrath::{
    DungeonDifficulty, Map,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/smsg_raid_instance_info.wowm:27`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/smsg_raid_instance_info.wowm#L27):
/// ```text
/// struct RaidInfo {
///     Map map;
///     (u32)DungeonDifficulty difficulty;
///     u64 instance_id;
///     Bool expired;
///     Bool extended;
///     u32 time_until_reset;
/// }
/// ```
pub struct RaidInfo {
    pub map: Map,
    pub difficulty: DungeonDifficulty,
    pub instance_id: u64,
    pub expired: bool,
    pub extended: bool,
    /// Seems to be in seconds
    ///
    pub time_until_reset: u32,
}

impl RaidInfo {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // map: Map
        w.write_all(&(self.map.as_int().to_le_bytes()))?;

        // difficulty: DungeonDifficulty
        w.write_all(&u32::from(self.difficulty.as_int()).to_le_bytes())?;

        // instance_id: u64
        w.write_all(&self.instance_id.to_le_bytes())?;

        // expired: Bool
        w.write_all(u8::from(self.expired).to_le_bytes().as_slice())?;

        // extended: Bool
        w.write_all(u8::from(self.extended).to_le_bytes().as_slice())?;

        // time_until_reset: u32
        w.write_all(&self.time_until_reset.to_le_bytes())?;

        Ok(())
    }
}

impl RaidInfo {
    pub(crate) fn read<R: Read>(mut r: R) -> Result<Self, crate::errors::ParseError> {
        // map: Map
        let map = crate::util::read_u32_le(&mut r)?.try_into()?;

        // difficulty: DungeonDifficulty
        let difficulty = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        // instance_id: u64
        let instance_id = crate::util::read_u64_le(&mut r)?;

        // expired: Bool
        let expired = crate::util::read_u8_le(&mut r)? != 0;

        // extended: Bool
        let extended = crate::util::read_u8_le(&mut r)? != 0;

        // time_until_reset: u32
        let time_until_reset = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            map,
            difficulty,
            instance_id,
            expired,
            extended,
            time_until_reset,
        })
    }

}

