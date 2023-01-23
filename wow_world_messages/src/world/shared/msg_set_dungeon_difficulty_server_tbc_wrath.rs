use std::convert::{TryFrom, TryInto};
use crate::world::shared::dungeon_difficulty_tbc_wrath::DungeonDifficulty;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/world/msg_set_dungeon_difficulty.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/world/msg_set_dungeon_difficulty.wowm#L1):
/// ```text
/// smsg MSG_SET_DUNGEON_DIFFICULTY_Server = 0x0329 {
///     (u32)DungeonDifficulty difficulty;
///     u32 unknown1;
///     Bool32 is_in_group;
/// }
/// ```
pub struct MSG_SET_DUNGEON_DIFFICULTY_Server {
    pub difficulty: DungeonDifficulty,
    /// ArcEmu hardcodes this to 1
    ///
    pub unknown1: u32,
    pub is_in_group: bool,
}

impl crate::Message for MSG_SET_DUNGEON_DIFFICULTY_Server {
    const OPCODE: u32 = 0x0329;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // difficulty: DungeonDifficulty
        w.write_all(&(self.difficulty.as_int() as u32).to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // is_in_group: Bool32
        w.write_all(u32::from(self.is_in_group).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0329, size: body_size as u32 });
        }

        // difficulty: DungeonDifficulty
        let difficulty: DungeonDifficulty = (crate::util::read_u32_le(r)? as u8).try_into()?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        // is_in_group: Bool32
        let is_in_group = crate::util::read_u32_le(r)? != 0;
        Ok(Self {
            difficulty,
            unknown1,
            is_in_group,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for MSG_SET_DUNGEON_DIFFICULTY_Server {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for MSG_SET_DUNGEON_DIFFICULTY_Server {}

