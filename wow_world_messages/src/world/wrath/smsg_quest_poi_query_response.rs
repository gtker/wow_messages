use std::io::{Read, Write};

use crate::wrath::QuestPoiList;

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

impl crate::private::Sealed for SMSG_QUEST_POI_QUERY_RESPONSE {}
impl crate::Message for SMSG_QUEST_POI_QUERY_RESPONSE {
    const OPCODE: u32 = 0x01e4;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_QUEST_POI_QUERY_RESPONSE {{").unwrap();
        // Members
        writeln!(s, "    amount_of_quests = {};", self.quests.len()).unwrap();
        write!(s, "    quests = [").unwrap();
        for v in self.quests.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        quest_id = {};", v.quest_id).unwrap();
            writeln!(s, "        amount_of_pois = {};", v.amount_of_pois).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 484_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_quests", "    ");
        if !self.quests.is_empty() {
            writeln!(s, "    /* quests: QuestPoiList[amount_of_quests] start */").unwrap();
            for (i, v) in self.quests.iter().enumerate() {
                writeln!(s, "    /* quests: QuestPoiList[amount_of_quests] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 4, "quest_id", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_pois", "        ");
                writeln!(s, "    /* quests: QuestPoiList[amount_of_quests] {i} end */").unwrap();
            }
            writeln!(s, "    /* quests: QuestPoiList[amount_of_quests] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // amount_of_quests: u32
        w.write_all(&(self.quests.len() as u32).to_le_bytes())?;

        // quests: QuestPoiList[amount_of_quests]
        for i in self.quests.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(4..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01E4, size: body_size });
        }

        // amount_of_quests: u32
        let amount_of_quests = crate::util::read_u32_le(&mut r)?;

        // quests: QuestPoiList[amount_of_quests]
        let quests = {
            let mut quests = Vec::with_capacity(amount_of_quests as usize);
            for _ in 0..amount_of_quests {
                quests.push(QuestPoiList::read(&mut r)?);
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

