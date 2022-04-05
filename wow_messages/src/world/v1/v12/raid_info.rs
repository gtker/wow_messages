use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Map, MapError};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new5.wowm:785`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new5.wowm#L785):
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

impl ReadableAndWritable for RaidInfo {
    type Error = RaidInfoError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // map: Map
        let map = Map::read(r)?;

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

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // map: Map
        self.map.write(w)?;

        // reset_time: u32
        w.write_all(&self.reset_time.to_le_bytes())?;

        // instance_id: u32
        w.write_all(&self.instance_id.to_le_bytes())?;

        Ok(())
    }

}

impl ConstantSized for RaidInfo {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for RaidInfo {
    fn maximum_possible_size() -> usize {
        Map::size() // map: Map
        + 4 // reset_time: u32
        + 4 // instance_id: u32
    }
}

#[derive(Debug)]
pub enum RaidInfoError {
    Io(std::io::Error),
    Map(MapError),
}

impl std::error::Error for RaidInfoError {}
impl std::fmt::Display for RaidInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Map(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for RaidInfoError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<MapError> for RaidInfoError {
    fn from(e: MapError) -> Self {
        Self::Map(e)
    }
}

