use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::LfgJoinLockedDungeon;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/smsg_lfg_join_result.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_lfg_join_result.wowm#L9):
/// ```text
/// struct LfgJoinPlayer {
///     Guid player;
///     u32 amount_of_locked_dungeons;
///     LfgJoinLockedDungeon[amount_of_locked_dungeons] locked_dungeons;
/// }
/// ```
pub struct LfgJoinPlayer {
    pub player: Guid,
    pub locked_dungeons: Vec<LfgJoinLockedDungeon>,
}

impl LfgJoinPlayer {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // player: Guid
        w.write_all(&self.player.guid().to_le_bytes())?;

        // amount_of_locked_dungeons: u32
        w.write_all(&(self.locked_dungeons.len() as u32).to_le_bytes())?;

        // locked_dungeons: LfgJoinLockedDungeon[amount_of_locked_dungeons]
        for i in self.locked_dungeons.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
}

impl LfgJoinPlayer {
    pub(crate) fn read<R: Read>(mut r: R) -> Result<Self, std::io::Error> {
        // player: Guid
        let player = crate::util::read_guid(&mut r)?;

        // amount_of_locked_dungeons: u32
        let amount_of_locked_dungeons = crate::util::read_u32_le(&mut r)?;

        // locked_dungeons: LfgJoinLockedDungeon[amount_of_locked_dungeons]
        let locked_dungeons = {
            let mut locked_dungeons = Vec::with_capacity(amount_of_locked_dungeons as usize);
            for _ in 0..amount_of_locked_dungeons {
                locked_dungeons.push(LfgJoinLockedDungeon::read(&mut r)?);
            }
            locked_dungeons
        };

        Ok(Self {
            player,
            locked_dungeons,
        })
    }

}

impl LfgJoinPlayer {
    pub(crate) fn size(&self) -> usize {
        8 // player: Guid
        + 4 // amount_of_locked_dungeons: u32
        + self.locked_dungeons.len() * 8 // locked_dungeons: LfgJoinLockedDungeon[amount_of_locked_dungeons]
    }
}

