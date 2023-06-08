use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::QuestItem;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_questgiver_quest_list.wowm:18`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_questgiver_quest_list.wowm#L18):
/// ```text
/// smsg SMSG_QUESTGIVER_QUEST_LIST = 0x0185 {
///     Guid npc;
///     CString title;
///     u32 emote_delay;
///     u32 emote;
///     u8 amount_of_entries;
///     QuestItem[amount_of_entries] quest_items;
/// }
/// ```
pub struct SMSG_QUESTGIVER_QUEST_LIST {
    pub npc: Guid,
    pub title: String,
    /// mangoszero: player emote
    pub emote_delay: u32,
    /// mangoszero: NPC emote
    pub emote: u32,
    pub quest_items: Vec<QuestItem>,
}

#[cfg(feature = "print-testcase")]
impl SMSG_QUESTGIVER_QUEST_LIST {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_QUESTGIVER_QUEST_LIST {{").unwrap();
        // Members
        writeln!(s, "    npc = {};", self.npc.guid()).unwrap();
        writeln!(s, "    title = \"{}\";", self.title).unwrap();
        writeln!(s, "    emote_delay = {};", self.emote_delay).unwrap();
        writeln!(s, "    emote = {};", self.emote).unwrap();
        writeln!(s, "    amount_of_entries = {};", self.quest_items.len()).unwrap();
        write!(s, "    quest_items = [").unwrap();
        for v in self.quest_items.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        quest_id = {};", v.quest_id).unwrap();
            writeln!(s, "        quest_icon = {};", v.quest_icon).unwrap();
            writeln!(s, "        level = {};", v.level.as_int()).unwrap();
            writeln!(s, "        flags = {};", v.flags).unwrap();
            writeln!(s, "        repeatable = {};", if v.repeatable { "TRUE" } else { "FALSE" }).unwrap();
            writeln!(s, "        title = \"{}\";", v.title).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 389_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "npc", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.title.len() + 1, "title", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "emote_delay", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "emote", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "amount_of_entries", "    ");
        if !self.quest_items.is_empty() {
            writeln!(s, "    /* quest_items: QuestItem[amount_of_entries] start */").unwrap();
            for (i, v) in self.quest_items.iter().enumerate() {
                writeln!(s, "    /* quest_items: QuestItem[amount_of_entries] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 4, "quest_id", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "quest_icon", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "level", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "flags", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "repeatable", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, v.title.len() + 1, "title", "        ");
                writeln!(s, "    /* quest_items: QuestItem[amount_of_entries] {i} end */").unwrap();
            }
            writeln!(s, "    /* quest_items: QuestItem[amount_of_entries] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_QUESTGIVER_QUEST_LIST {}
impl crate::Message for SMSG_QUESTGIVER_QUEST_LIST {
    const OPCODE: u32 = 0x0185;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_QUESTGIVER_QUEST_LIST::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // npc: Guid
        w.write_all(&self.npc.guid().to_le_bytes())?;

        // title: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.title.as_bytes().iter().rev().next(), Some(&0_u8), "String `title` must not be null-terminated.");
        w.write_all(self.title.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // emote_delay: u32
        w.write_all(&self.emote_delay.to_le_bytes())?;

        // emote: u32
        w.write_all(&self.emote.to_le_bytes())?;

        // amount_of_entries: u8
        w.write_all(&(self.quest_items.len() as u8).to_le_bytes())?;

        // quest_items: QuestItem[amount_of_entries]
        for i in self.quest_items.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(18..=70161).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0185, size: body_size });
        }

        // npc: Guid
        let npc = crate::util::read_guid(&mut r)?;

        // title: CString
        let title = {
            let title = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(title)?
        };

        // emote_delay: u32
        let emote_delay = crate::util::read_u32_le(&mut r)?;

        // emote: u32
        let emote = crate::util::read_u32_le(&mut r)?;

        // amount_of_entries: u8
        let amount_of_entries = crate::util::read_u8_le(&mut r)?;

        // quest_items: QuestItem[amount_of_entries]
        let quest_items = {
            let mut quest_items = Vec::with_capacity(amount_of_entries as usize);
            for _ in 0..amount_of_entries {
                quest_items.push(QuestItem::read(&mut r)?);
            }
            quest_items
        };

        Ok(Self {
            npc,
            title,
            emote_delay,
            emote,
            quest_items,
        })
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_QUESTGIVER_QUEST_LIST {}

impl SMSG_QUESTGIVER_QUEST_LIST {
    pub(crate) fn size(&self) -> usize {
        8 // npc: Guid
        + self.title.len() + 1 // title: CString
        + 4 // emote_delay: u32
        + 4 // emote: u32
        + 1 // amount_of_entries: u8
        + self.quest_items.iter().fold(0, |acc, x| acc + x.size()) // quest_items: QuestItem[amount_of_entries]
    }
}

