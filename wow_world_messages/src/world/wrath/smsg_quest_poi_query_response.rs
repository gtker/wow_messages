use std::io::{Read, Write};

use crate::wrath::{
    Area, Map, QuestPoi, QuestPoiList, Vector2dUnsigned,
};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_quest_poi_query_response.wowm:30`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_quest_poi_query_response.wowm#L30):
/// ```text
/// smsg SMSG_QUEST_POI_QUERY_RESPONSE = 0x01E4 {
///     u32 amount_of_quests;
///     QuestPoiList[amount_of_quests] quests;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_QUEST_POI_QUERY_RESPONSE {
    pub quests: Vec<QuestPoiList>,
}

impl crate::private::Sealed for SMSG_QUEST_POI_QUERY_RESPONSE {}
impl SMSG_QUEST_POI_QUERY_RESPONSE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(4..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // amount_of_quests: u32
        let amount_of_quests = crate::util::read_u32_le(&mut r)?;

        // quests: QuestPoiList[amount_of_quests]
        let quests = {
            let mut quests = Vec::with_capacity(amount_of_quests as usize);

            let allocation_size = u64::from(amount_of_quests) * 8;
            if allocation_size > crate::errors::MAX_ALLOCATION_SIZE_WRATH {
                return Err(crate::errors::ParseErrorKind::AllocationTooLargeError(allocation_size));
            }

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

impl crate::Message for SMSG_QUEST_POI_QUERY_RESPONSE {
    const OPCODE: u32 = 0x01e4;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_QUEST_POI_QUERY_RESPONSE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_QUEST_POI_QUERY_RESPONSE {{").unwrap();
        // Members
        writeln!(s, "    amount_of_quests = {};", self.quests.len()).unwrap();
        writeln!(s, "    quests = [").unwrap();
        for v in self.quests.as_slice() {
            writeln!(s, "        {{").unwrap();
            // Members
            writeln!(s, "            quest_id = {};", v.quest_id).unwrap();
            writeln!(s, "            amount_of_pois = {};", v.pois.len()).unwrap();
            writeln!(s, "            pois = [").unwrap();
            for v in v.pois.as_slice() {
                writeln!(s, "                {{").unwrap();
                // Members
                writeln!(s, "                    id = {};", v.id).unwrap();
                writeln!(s, "                    objective_id = {};", v.objective_id).unwrap();
                writeln!(s, "                    map = {};", v.map.as_test_case_value()).unwrap();
                writeln!(s, "                    area = {};", v.area.as_test_case_value()).unwrap();
                writeln!(s, "                    floor_id = {};", v.floor_id).unwrap();
                writeln!(s, "                    unknown1 = {};", v.unknown1).unwrap();
                writeln!(s, "                    unknown2 = {};", v.unknown2).unwrap();
                writeln!(s, "                    amount_of_points = {};", v.points.len()).unwrap();
                writeln!(s, "                    points = [").unwrap();
                for v in v.points.as_slice() {
                    writeln!(s, "                        {{").unwrap();
                    // Members
                    writeln!(s, "                            x = {};", v.x).unwrap();
                    writeln!(s, "                            y = {};", v.y).unwrap();

                    writeln!(s, "                        }},").unwrap();
                }
                writeln!(s, "                    ];").unwrap();

                writeln!(s, "                }},").unwrap();
            }
            writeln!(s, "            ];").unwrap();

            writeln!(s, "        }},").unwrap();
        }
        writeln!(s, "    ];").unwrap();

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
                if !v.pois.is_empty() {
                    writeln!(s, "    /* pois: QuestPoi[amount_of_pois] start */").unwrap();
                    for (i, v) in v.pois.iter().enumerate() {
                        writeln!(s, "    /* pois: QuestPoi[amount_of_pois] {i} start */").unwrap();
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "id", "            ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "objective_id", "            ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "map", "            ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "area", "            ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "floor_id", "            ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown1", "            ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown2", "            ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_points", "            ");
                        if !v.points.is_empty() {
                            writeln!(s, "    /* points: Vector2dUnsigned[amount_of_points] start */").unwrap();
                            for (i, v) in v.points.iter().enumerate() {
                                writeln!(s, "    /* points: Vector2dUnsigned[amount_of_points] {i} start */").unwrap();
                                crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "                ");
                                crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "                ");
                                writeln!(s, "    /* points: Vector2dUnsigned[amount_of_points] {i} end */").unwrap();
                            }
                            writeln!(s, "    /* points: Vector2dUnsigned[amount_of_points] end */").unwrap();
                        }
                        writeln!(s, "    /* pois: QuestPoi[amount_of_pois] {i} end */").unwrap();
                    }
                    writeln!(s, "    /* pois: QuestPoi[amount_of_pois] end */").unwrap();
                }
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

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(484, "SMSG_QUEST_POI_QUERY_RESPONSE", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_QUEST_POI_QUERY_RESPONSE {}

impl SMSG_QUEST_POI_QUERY_RESPONSE {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_quests: u32
        + self.quests.iter().fold(0, |acc, x| acc + x.size()) // quests: QuestPoiList[amount_of_quests]
    }
}

