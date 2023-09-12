use std::io::{Read, Write};

use crate::Guid;
use crate::shared::level_vanilla_tbc_wrath::Level;
use crate::vanilla::{
    GossipItem, QuestItem,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gossip/smsg_gossip_message.wowm:14`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gossip/smsg_gossip_message.wowm#L14):
/// ```text
/// smsg SMSG_GOSSIP_MESSAGE = 0x017D {
///     Guid guid;
///     u32 title_text_id;
///     u32 amount_of_gossip_items;
///     GossipItem[amount_of_gossip_items] gossips;
///     u32 amount_of_quests;
///     QuestItem[amount_of_quests] quests;
/// }
/// ```
pub struct SMSG_GOSSIP_MESSAGE {
    pub guid: Guid,
    pub title_text_id: u32,
    pub gossips: Vec<GossipItem>,
    pub quests: Vec<QuestItem>,
}

impl crate::private::Sealed for SMSG_GOSSIP_MESSAGE {}
impl SMSG_GOSSIP_MESSAGE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(20..=65535).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // title_text_id: u32
        let title_text_id = crate::util::read_u32_le(&mut r)?;

        // amount_of_gossip_items: u32
        let amount_of_gossip_items = crate::util::read_u32_le(&mut r)?;

        // gossips: GossipItem[amount_of_gossip_items]
        let gossips = {
            let mut gossips = Vec::with_capacity(amount_of_gossip_items as usize);
            for _ in 0..amount_of_gossip_items {
                gossips.push(GossipItem::read(&mut r)?);
            }
            gossips
        };

        // amount_of_quests: u32
        let amount_of_quests = crate::util::read_u32_le(&mut r)?;

        // quests: QuestItem[amount_of_quests]
        let quests = {
            let mut quests = Vec::with_capacity(amount_of_quests as usize);
            for _ in 0..amount_of_quests {
                quests.push(QuestItem::read(&mut r)?);
            }
            quests
        };

        Ok(Self {
            guid,
            title_text_id,
            gossips,
            quests,
        })
    }

}

impl crate::Message for SMSG_GOSSIP_MESSAGE {
    const OPCODE: u32 = 0x017d;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_GOSSIP_MESSAGE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_GOSSIP_MESSAGE {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    title_text_id = {};", self.title_text_id).unwrap();
        writeln!(s, "    amount_of_gossip_items = {};", self.gossips.len()).unwrap();
        write!(s, "    gossips = [").unwrap();
        for v in self.gossips.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        id = {};", v.id).unwrap();
            writeln!(s, "        item_icon = {};", v.item_icon).unwrap();
            writeln!(s, "        coded = {};", if v.coded { "TRUE" } else { "FALSE" }).unwrap();
            writeln!(s, "        message = \"{}\";", v.message).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();
        writeln!(s, "    amount_of_quests = {};", self.quests.len()).unwrap();
        write!(s, "    quests = [").unwrap();
        for v in self.quests.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        quest_id = {};", v.quest_id).unwrap();
            writeln!(s, "        quest_icon = {};", v.quest_icon).unwrap();
            writeln!(s, "        level = {};", v.level.as_int()).unwrap();
            writeln!(s, "        title = \"{}\";", v.title).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 381_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "title_text_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_gossip_items", "    ");
        if !self.gossips.is_empty() {
            writeln!(s, "    /* gossips: GossipItem[amount_of_gossip_items] start */").unwrap();
            for (i, v) in self.gossips.iter().enumerate() {
                writeln!(s, "    /* gossips: GossipItem[amount_of_gossip_items] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 4, "id", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "item_icon", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "coded", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, v.message.len() + 1, "message", "        ");
                writeln!(s, "    /* gossips: GossipItem[amount_of_gossip_items] {i} end */").unwrap();
            }
            writeln!(s, "    /* gossips: GossipItem[amount_of_gossip_items] end */").unwrap();
        }
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_quests", "    ");
        if !self.quests.is_empty() {
            writeln!(s, "    /* quests: QuestItem[amount_of_quests] start */").unwrap();
            for (i, v) in self.quests.iter().enumerate() {
                writeln!(s, "    /* quests: QuestItem[amount_of_quests] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 4, "quest_id", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "quest_icon", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "level", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, v.title.len() + 1, "title", "        ");
                writeln!(s, "    /* quests: QuestItem[amount_of_quests] {i} end */").unwrap();
            }
            writeln!(s, "    /* quests: QuestItem[amount_of_quests] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1.12".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // title_text_id: u32
        w.write_all(&self.title_text_id.to_le_bytes())?;

        // amount_of_gossip_items: u32
        w.write_all(&(self.gossips.len() as u32).to_le_bytes())?;

        // gossips: GossipItem[amount_of_gossip_items]
        for i in self.gossips.iter() {
            i.write_into_vec(&mut w)?;
        }

        // amount_of_quests: u32
        w.write_all(&(self.quests.len() as u32).to_le_bytes())?;

        // quests: QuestItem[amount_of_quests]
        for i in self.quests.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(381, "SMSG_GOSSIP_MESSAGE", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_GOSSIP_MESSAGE {}

impl SMSG_GOSSIP_MESSAGE {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // title_text_id: u32
        + 4 // amount_of_gossip_items: u32
        + self.gossips.iter().fold(0, |acc, x| acc + x.size()) // gossips: GossipItem[amount_of_gossip_items]
        + 4 // amount_of_quests: u32
        + self.quests.iter().fold(0, |acc, x| acc + x.size()) // quests: QuestItem[amount_of_quests]
    }
}

