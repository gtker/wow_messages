use crate::wrath::LfgAvailableDungeon;
use crate::wrath::LfgJoinLockedDungeon;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/smsg_lfg_player_info.wowm:22`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_lfg_player_info.wowm#L22):
/// ```text
/// smsg SMSG_LFG_PLAYER_INFO = 0x036F {
///     u8 amount_of_available_dungeons;
///     LfgAvailableDungeon[amount_of_available_dungeons] available_dungeons;
///     u8 amount_of_locked_dungeons;
///     LfgJoinLockedDungeon[amount_of_locked_dungeons] locked_dungeons;
/// }
/// ```
pub struct SMSG_LFG_PLAYER_INFO {
    pub available_dungeons: Vec<LfgAvailableDungeon>,
    pub locked_dungeons: Vec<LfgJoinLockedDungeon>,
}

impl crate::Message for SMSG_LFG_PLAYER_INFO {
    const OPCODE: u32 = 0x036f;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // amount_of_available_dungeons: u8
        w.write_all(&(self.available_dungeons.len() as u8).to_le_bytes())?;

        // available_dungeons: LfgAvailableDungeon[amount_of_available_dungeons]
        for i in self.available_dungeons.iter() {
            i.write_into_vec(w)?;
        }

        // amount_of_locked_dungeons: u8
        w.write_all(&(self.locked_dungeons.len() as u8).to_le_bytes())?;

        // locked_dungeons: LfgJoinLockedDungeon[amount_of_locked_dungeons]
        for i in self.locked_dungeons.iter() {
            i.write_into_vec(w)?;
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(2..=794114).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x036F, size: body_size as u32 });
        }

        // amount_of_available_dungeons: u8
        let amount_of_available_dungeons = crate::util::read_u8_le(r)?;

        // available_dungeons: LfgAvailableDungeon[amount_of_available_dungeons]
        let mut available_dungeons = Vec::with_capacity(amount_of_available_dungeons as usize);
        for i in 0..amount_of_available_dungeons {
            available_dungeons.push(LfgAvailableDungeon::read(r)?);
        }

        // amount_of_locked_dungeons: u8
        let amount_of_locked_dungeons = crate::util::read_u8_le(r)?;

        // locked_dungeons: LfgJoinLockedDungeon[amount_of_locked_dungeons]
        let mut locked_dungeons = Vec::with_capacity(amount_of_locked_dungeons as usize);
        for i in 0..amount_of_locked_dungeons {
            locked_dungeons.push(LfgJoinLockedDungeon::read(r)?);
        }

        Ok(Self {
            available_dungeons,
            locked_dungeons,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_LFG_PLAYER_INFO {}

impl SMSG_LFG_PLAYER_INFO {
    pub(crate) fn size(&self) -> usize {
        1 // amount_of_available_dungeons: u8
        + self.available_dungeons.iter().fold(0, |acc, x| acc + x.size()) // available_dungeons: LfgAvailableDungeon[amount_of_available_dungeons]
        + 1 // amount_of_locked_dungeons: u8
        + self.locked_dungeons.len() * 8 // locked_dungeons: LfgJoinLockedDungeon[amount_of_locked_dungeons]
    }
}

