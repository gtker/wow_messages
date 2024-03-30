use std::io::{Read, Write};

use crate::wrath::{
    Area, Map, Vector2dUnsigned,
};

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
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
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
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // objective_id: u32
        w.write_all(&self.objective_id.to_le_bytes())?;

        // map: Map
        w.write_all(&(self.map.as_int().to_le_bytes()))?;

        // area: Area
        w.write_all(&(self.area.as_int().to_le_bytes()))?;

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
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
}

impl QuestPoi {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // id: u32
        let id = crate::util::read_u32_le(&mut r)?;

        // objective_id: u32
        let objective_id = crate::util::read_u32_le(&mut r)?;

        // map: Map
        let map = crate::util::read_u32_le(&mut r)?.try_into()?;

        // area: Area
        let area = crate::util::read_u32_le(&mut r)?.try_into()?;

        // floor_id: u32
        let floor_id = crate::util::read_u32_le(&mut r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(&mut r)?;

        // unknown2: u32
        let unknown2 = crate::util::read_u32_le(&mut r)?;

        // amount_of_points: u32
        let amount_of_points = crate::util::read_u32_le(&mut r)?;

        // points: Vector2dUnsigned[amount_of_points]
        let points = {
            let mut points = Vec::with_capacity(amount_of_points as usize);

            let allocation_size = u64::from(amount_of_points) * 8;
            if allocation_size > crate::errors::MAX_ALLOCATION_SIZE_WRATH {
                return Err(crate::errors::ParseErrorKind::AllocationTooLargeError(allocation_size));
            }

            for _ in 0..amount_of_points {
                points.push(Vector2dUnsigned::read(&mut r)?);
            }
            points
        };

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

