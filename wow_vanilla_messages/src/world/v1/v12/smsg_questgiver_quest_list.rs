use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::QuestItem;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_QUESTGIVER_QUEST_LIST {
    pub npc: Guid,
    pub title: String,
    pub emote_delay: u32,
    pub emote: u32,
    pub quest_items: Vec<QuestItem>,
}

impl SMSG_QUESTGIVER_QUEST_LIST {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // npc: Guid
        w.write_all(&self.npc.guid().to_le_bytes())?;

        // title: CString
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
            w.write_all(&(i.as_bytes()?))?;
        }

        Ok(w)
    }
}

impl ServerMessage for SMSG_QUESTGIVER_QUEST_LIST {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // npc: Guid
        w.write_all(&self.npc.guid().to_le_bytes())?;

        // title: CString
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
            w.write_all(&(i.as_bytes()?))?;
        }

        Ok(())
    }
    const OPCODE: u16 = 0x0185;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = crate::errors::ParseError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // npc: Guid
        let npc = Guid::read(r)?;

        // title: CString
        let title = crate::util::read_c_string_to_vec(r)?;
        let title = String::from_utf8(title)?;

        // emote_delay: u32
        let emote_delay = crate::util::read_u32_le(r)?;

        // emote: u32
        let emote = crate::util::read_u32_le(r)?;

        // amount_of_entries: u8
        let amount_of_entries = crate::util::read_u8_le(r)?;

        // quest_items: QuestItem[amount_of_entries]
        let mut quest_items = Vec::with_capacity(amount_of_entries as usize);
        for i in 0..amount_of_entries {
            quest_items.push(QuestItem::read(r)?);
        }

        Ok(Self {
            npc,
            title,
            emote_delay,
            emote,
            quest_items,
        })
    }

}

impl SMSG_QUESTGIVER_QUEST_LIST {
    pub fn size(&self) -> usize {
        0
        + 8 // npc: Guid
        + self.title.len() + 1 // title: CString
        + 4 // emote_delay: u32
        + 4 // emote: u32
        + 1 // amount_of_entries: u8
        + self.quest_items.iter().fold(0, |acc, x| acc + x.size()) // quest_items: QuestItem[amount_of_entries]
    }
}

