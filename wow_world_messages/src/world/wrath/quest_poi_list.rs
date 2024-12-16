use std::io::{Read, Write};

use crate::wrath::{
    Area, Map, QuestPoi, Vector2dUnsigned,
};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_quest_poi_query_response.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_quest_poi_query_response.wowm#L1):
/// ```text
/// struct QuestPoiList {
///     u32 quest_id;
///     u32 amount_of_pois;
///     QuestPoi[amount_of_pois] pois;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct QuestPoiList {
    pub quest_id: u32,
    pub pois: Vec<QuestPoi>,
}

impl QuestPoiList {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // amount_of_pois: u32
        w.write_all(&(self.pois.len() as u32).to_le_bytes())?;

        // pois: QuestPoi[amount_of_pois]
        for i in self.pois.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
}

impl QuestPoiList {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // quest_id: u32
        let quest_id = crate::util::read_u32_le(&mut r)?;

        // amount_of_pois: u32
        let amount_of_pois = crate::util::read_u32_le(&mut r)?;

        // pois: QuestPoi[amount_of_pois]
        let pois = {
            let mut pois = Vec::with_capacity(amount_of_pois as usize);

            let allocation_size = u64::from(amount_of_pois) * 32;
            if allocation_size > crate::errors::MAX_ALLOCATION_SIZE_WRATH {
                return Err(crate::errors::ParseErrorKind::AllocationTooLargeError(allocation_size));
            }

            for _ in 0..amount_of_pois {
                pois.push(QuestPoi::read(&mut r)?);
            }
            pois
        };

        Ok(Self {
            quest_id,
            pois,
        })
    }

}

impl QuestPoiList {
    pub(crate) fn size(&self) -> usize {
        4 // quest_id: u32
        + 4 // amount_of_pois: u32
        + self.pois.iter().fold(0, |acc, x| acc + x.size()) // pois: QuestPoi[amount_of_pois]
    }
}

