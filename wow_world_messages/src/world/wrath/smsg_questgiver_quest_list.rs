use crate::Guid;
use crate::wrath::QuestItem;
use std::io::{Write, Read};

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
    ///
    pub emote_delay: u32,
    /// mangoszero: NPC emote
    ///
    pub emote: u32,
    pub quest_items: Vec<QuestItem>,
}

impl crate::Message for SMSG_QUESTGIVER_QUEST_LIST {
    const OPCODE: u32 = 0x0185;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
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
            i.write_into_vec(w)?;
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(18..=70161).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0185, size: body_size as u32 });
        }

        // npc: Guid
        let npc = Guid::read(r)?;

        // title: CString
        let title = {
            let title = crate::util::read_c_string_to_vec(r)?;
            String::from_utf8(title)?
        };

        // emote_delay: u32
        let emote_delay = crate::util::read_u32_le(r)?;

        // emote: u32
        let emote = crate::util::read_u32_le(r)?;

        // amount_of_entries: u8
        let amount_of_entries = crate::util::read_u8_le(r)?;

        // quest_items: QuestItem[amount_of_entries]
        let quest_items = {
            let mut quest_items = Vec::with_capacity(amount_of_entries as usize);
            for i in 0..amount_of_entries {
                quest_items.push(QuestItem::read(r)?);
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

