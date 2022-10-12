use std::convert::{TryFrom, TryInto};
use crate::world::wrath::DungeonDifficulty;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/world/msg_set_dungeon_difficulty.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/world/msg_set_dungeon_difficulty.wowm#L9):
/// ```text
/// msg MSG_SET_DUNGEON_DIFFICULTY = 0x0329 {
///     DungeonDifficulty difficulty;
///     u32 unknown1;
///     u32 is_in_group;
/// }
/// ```
pub struct MSG_SET_DUNGEON_DIFFICULTY {
    pub difficulty: DungeonDifficulty,
    /// ArcEmu hardcodes this to 1
    ///
    pub unknown1: u32,
    pub is_in_group: u32,
}

impl crate::Message for MSG_SET_DUNGEON_DIFFICULTY {
    const OPCODE: u32 = 0x0329;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // difficulty: DungeonDifficulty
        w.write_all(&(self.difficulty.as_int() as u32).to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // is_in_group: u32
        w.write_all(&self.is_in_group.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // difficulty: DungeonDifficulty
        let difficulty: DungeonDifficulty = crate::util::read_u32_le(r)?.try_into()?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        // is_in_group: u32
        let is_in_group = crate::util::read_u32_le(r)?;

        Ok(Self {
            difficulty,
            unknown1,
            is_in_group,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for MSG_SET_DUNGEON_DIFFICULTY {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for MSG_SET_DUNGEON_DIFFICULTY {}

