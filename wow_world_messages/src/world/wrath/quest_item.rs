use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/common.wowm:746`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/common.wowm#L746):
/// ```text
/// struct QuestItem {
///     u32 quest_id;
///     u32 quest_icon;
///     u32 level;
///     u32 flags;
///     Bool repeatable;
///     CString title;
/// }
/// ```
pub struct QuestItem {
    pub quest_id: u32,
    pub quest_icon: u32,
    pub level: u32,
    pub flags: u32,
    pub repeatable: bool,
    /// vmangos/cmangos/mangoszero: max 0x200
    ///
    pub title: String,
}

impl QuestItem {
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // quest_icon: u32
        w.write_all(&self.quest_icon.to_le_bytes())?;

        // level: u32
        w.write_all(&self.level.to_le_bytes())?;

        // flags: u32
        w.write_all(&self.flags.to_le_bytes())?;

        // repeatable: Bool
        w.write_all(u8::from(self.repeatable).to_le_bytes().as_slice())?;

        // title: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.title.as_bytes().iter().rev().next(), Some(&0_u8), "String `title` must not be null-terminated.");
        w.write_all(self.title.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
}

impl QuestItem {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> std::result::Result<Self, crate::errors::ParseError> {
        // quest_id: u32
        let quest_id = crate::util::read_u32_le(&mut r)?;

        // quest_icon: u32
        let quest_icon = crate::util::read_u32_le(&mut r)?;

        // level: u32
        let level = crate::util::read_u32_le(&mut r)?;

        // flags: u32
        let flags = crate::util::read_u32_le(&mut r)?;

        // repeatable: Bool
        let repeatable = crate::util::read_u8_le(&mut r)? != 0;

        // title: CString
        let title = {
            let title = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(title)?
        };

        Ok(Self {
            quest_id,
            quest_icon,
            level,
            flags,
            repeatable,
            title,
        })
    }

}

impl QuestItem {
    pub(crate) fn size(&self) -> usize {
        4 // quest_id: u32
        + 4 // quest_icon: u32
        + 4 // level: u32
        + 4 // flags: u32
        + 1 // repeatable: Bool
        + self.title.len() + 1 // title: CString
    }
}

