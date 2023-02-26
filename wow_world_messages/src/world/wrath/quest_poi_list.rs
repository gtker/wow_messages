use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_quest_poi_query_response.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_quest_poi_query_response.wowm#L1):
/// ```text
/// struct QuestPoiList {
///     u32 quest_id;
///     u32 amount_of_pois;
/// }
/// ```
pub struct QuestPoiList {
    pub quest_id: u32,
    pub amount_of_pois: u32,
}

impl QuestPoiList {
    pub(crate) fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // amount_of_pois: u32
        w.write_all(&self.amount_of_pois.to_le_bytes())?;

        Ok(())
    }
}

impl QuestPoiList {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // quest_id: u32
        let quest_id = crate::util::read_u32_le(r)?;

        // amount_of_pois: u32
        let amount_of_pois = crate::util::read_u32_le(r)?;

        Ok(Self {
            quest_id,
            amount_of_pois,
        })
    }

}

