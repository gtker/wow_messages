use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/common.wowm:268`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/common.wowm#L268):
/// ```text
/// struct QuestItem {
///     u32 quest_id;
///     u32 quest_icon;
///     u32 level;
///     CString title;
/// }
/// ```
pub struct QuestItem {
    pub quest_id: u32,
    pub quest_icon: u32,
    pub level: u32,
    /// vmangos/cmangos/mangoszero: max 0x200
    ///
    pub title: String,
}

impl QuestItem {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // quest_icon: u32
        w.write_all(&self.quest_icon.to_le_bytes())?;

        // level: u32
        w.write_all(&self.level.to_le_bytes())?;

        // title: CString
        w.write_all(self.title.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
}

impl QuestItem {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // quest_id: u32
        let quest_id = crate::util::read_u32_le(r)?;

        // quest_icon: u32
        let quest_icon = crate::util::read_u32_le(r)?;

        // level: u32
        let level = crate::util::read_u32_le(r)?;

        // title: CString
        let title = crate::util::read_c_string_to_vec(r)?;
        let title = String::from_utf8(title)?;

        Ok(Self {
            quest_id,
            quest_icon,
            level,
            title,
        })
    }

}

impl QuestItem {
    pub(crate) fn size(&self) -> usize {
        4 // quest_id: u32
        + 4 // quest_icon: u32
        + 4 // level: u32
        + self.title.len() + 1 // title: CString
    }
}

