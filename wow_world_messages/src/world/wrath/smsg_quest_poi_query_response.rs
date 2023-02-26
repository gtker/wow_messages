use crate::wrath::QuestPoiList;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_quest_poi_query_response.wowm:30`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_quest_poi_query_response.wowm#L30):
/// ```text
/// smsg SMSG_QUEST_POI_QUERY_RESPONSE = 0x01E4 {
///     u32 amount_of_quests;
///     QuestPoiList[amount_of_quests] quests;
/// }
/// ```
pub struct SMSG_QUEST_POI_QUERY_RESPONSE {
    pub quests: Vec<QuestPoiList>,
}

impl crate::Message for SMSG_QUEST_POI_QUERY_RESPONSE {
    const OPCODE: u32 = 0x01e4;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // amount_of_quests: u32
        w.write_all(&(self.quests.len() as u32).to_le_bytes())?;

        // quests: QuestPoiList[amount_of_quests]
        for i in self.quests.iter() {
            i.write_into_vec(w)?;
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(4..=4294967294).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01E4, size: body_size as u32 });
        }

        // amount_of_quests: u32
        let amount_of_quests = crate::util::read_u32_le(r)?;

        // quests: QuestPoiList[amount_of_quests]
        let quests = {
            let mut quests = Vec::with_capacity(amount_of_quests as usize);
            for i in 0..amount_of_quests {
                quests.push(QuestPoiList::read(r)?);
            }
            quests
        };

        Ok(Self {
            quests,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_QUEST_POI_QUERY_RESPONSE {}

impl SMSG_QUEST_POI_QUERY_RESPONSE {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_quests: u32
        + self.quests.len() * 8 // quests: QuestPoiList[amount_of_quests]
    }
}

