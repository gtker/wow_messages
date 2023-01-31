use std::convert::{TryFrom, TryInto};
use crate::wrath::Vector2dUnsigned;
use crate::wrath::Area;
use crate::wrath::Map;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_quest_poi_query_response.wowm:16`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_quest_poi_query_response.wowm#L16):
/// ```text
/// struct QuestPoi {
///     u32 id;
///     u32 objective_id;
///     Map map;
///     Area area;
///     u32 floor_id;
///     u32 unknown1;
///     u32 unknown2;
///     u32 amount_of_points;
///     Vector2dUnsigned[amount_of_points] points;
/// }
/// ```
pub struct QuestPoi {
    pub id: u32,
    pub objective_id: u32,
    pub map: Map,
    pub area: Area,
    pub floor_id: u32,
    pub unknown1: u32,
    pub unknown2: u32,
    pub points: Vec<Vector2dUnsigned>,
}

impl QuestPoi {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // objective_id: u32
        w.write_all(&self.objective_id.to_le_bytes())?;

        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        // area: Area
        w.write_all(&(self.area.as_int() as u32).to_le_bytes())?;

        // floor_id: u32
        w.write_all(&self.floor_id.to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: u32
        w.write_all(&self.unknown2.to_le_bytes())?;

        // amount_of_points: u32
        w.write_all(&(self.points.len() as u32).to_le_bytes())?;

        // points: Vector2dUnsigned[amount_of_points]
        for i in self.points.iter() {
            i.write_into_vec(w)?;
        }

        Ok(())
    }
}

impl QuestPoi {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // id: u32
        let id = crate::util::read_u32_le(r)?;

        // objective_id: u32
        let objective_id = crate::util::read_u32_le(r)?;

        // map: Map
        let map: Map = crate::util::read_u32_le(r)?.try_into()?;

        // area: Area
        let area: Area = crate::util::read_u32_le(r)?.try_into()?;

        // floor_id: u32
        let floor_id = crate::util::read_u32_le(r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        // unknown2: u32
        let unknown2 = crate::util::read_u32_le(r)?;

        // amount_of_points: u32
        let amount_of_points = crate::util::read_u32_le(r)?;

        // points: Vector2dUnsigned[amount_of_points]
        let mut points = Vec::with_capacity(amount_of_points as usize);
        for i in 0..amount_of_points {
            points.push(Vector2dUnsigned::read(r)?);
        }

        Ok(Self {
            id,
            objective_id,
            map,
            area,
            floor_id,
            unknown1,
            unknown2,
            points,
        })
    }

}

impl QuestPoi {
    pub(crate) fn size(&self) -> usize {
        4 // id: u32
        + 4 // objective_id: u32
        + 4 // map: Map
        + 4 // area: Area
        + 4 // floor_id: u32
        + 4 // unknown1: u32
        + 4 // unknown2: u32
        + 4 // amount_of_points: u32
        + self.points.len() * 8 // points: Vector2dUnsigned[amount_of_points]
    }
}

