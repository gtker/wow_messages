use std::convert::{TryFrom, TryInto};
use crate::world::shared::dungeon_difficulty_tbc_wrath::DungeonDifficulty;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/world/msg_set_dungeon_difficulty.wowm:12`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/world/msg_set_dungeon_difficulty.wowm#L12):
/// ```text
/// cmsg MSG_SET_DUNGEON_DIFFICULTY_Client = 0x0329 {
///     (u32)DungeonDifficulty difficulty;
/// }
/// ```
pub struct MSG_SET_DUNGEON_DIFFICULTY_Client {
    pub difficulty: DungeonDifficulty,
}

impl crate::Message for MSG_SET_DUNGEON_DIFFICULTY_Client {
    const OPCODE: u32 = 0x0329;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // difficulty: DungeonDifficulty
        w.write_all(&(self.difficulty.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0329, size: body_size as u32 });
        }

        // difficulty: DungeonDifficulty
        let difficulty: DungeonDifficulty = (crate::util::read_u32_le(r)? as u8).try_into()?;

        Ok(Self {
            difficulty,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for MSG_SET_DUNGEON_DIFFICULTY_Client {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for MSG_SET_DUNGEON_DIFFICULTY_Client {}

