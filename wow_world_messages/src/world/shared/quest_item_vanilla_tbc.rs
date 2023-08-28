use std::io::{Read, Write};

use wow_world_base::shared::level_vanilla_tbc_wrath::Level;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/common.wowm:849`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/common.wowm#L849):
/// ```text
/// struct QuestItem {
///     u32 quest_id;
///     u32 quest_icon;
///     Level32 level;
///     CString title;
/// }
/// ```
pub struct QuestItem {
    pub quest_id: u32,
    pub quest_icon: u32,
    pub level: Level,
    /// vmangos/cmangos/mangoszero: max 0x200
    pub title: String,
}

impl QuestItem {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // quest_icon: u32
        w.write_all(&self.quest_icon.to_le_bytes())?;

        // level: Level32
        w.write_all(&u32::from(self.level.as_int()).to_le_bytes())?;

        // title: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.title.as_bytes().iter().next_back(), Some(&0_u8), "String `title` must not be null-terminated.");
        w.write_all(self.title.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
}

impl QuestItem {
    pub(crate) fn read<R: Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // quest_id: u32
        let quest_id = crate::util::read_u32_le(&mut r)?;

        // quest_icon: u32
        let quest_icon = crate::util::read_u32_le(&mut r)?;

        // level: Level32
        let level = Level::new(crate::util::read_u32_le(&mut r)? as u8);

        // title: CString
        let title = {
            let title = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(title)?
        };

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
        + 4 // level: Level32
        + self.title.len() + 1 // title: CString
    }
}

