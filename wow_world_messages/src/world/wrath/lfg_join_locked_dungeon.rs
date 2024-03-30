use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/smsg_lfg_join_result.wowm:17`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_lfg_join_result.wowm#L17):
/// ```text
/// struct LfgJoinLockedDungeon {
///     u32 dungeon_entry;
///     u32 reason;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct LfgJoinLockedDungeon {
    pub dungeon_entry: u32,
    pub reason: u32,
}

impl LfgJoinLockedDungeon {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // dungeon_entry: u32
        w.write_all(&self.dungeon_entry.to_le_bytes())?;

        // reason: u32
        w.write_all(&self.reason.to_le_bytes())?;

        Ok(())
    }
}

impl LfgJoinLockedDungeon {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, std::io::Error> {
        // dungeon_entry: u32
        let dungeon_entry = crate::util::read_u32_le(&mut r)?;

        // reason: u32
        let reason = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            dungeon_entry,
            reason,
        })
    }

}

