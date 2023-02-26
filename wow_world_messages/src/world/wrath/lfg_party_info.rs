use crate::Guid;
use crate::wrath::LfgJoinLockedDungeon;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/smsg_lfg_party_info.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_lfg_party_info.wowm#L1):
/// ```text
/// struct LfgPartyInfo {
///     Guid player;
///     u32 amount_of_dungeons;
///     LfgJoinLockedDungeon[amount_of_dungeons] dungeons;
/// }
/// ```
pub struct LfgPartyInfo {
    pub player: Guid,
    pub dungeons: Vec<LfgJoinLockedDungeon>,
}

impl LfgPartyInfo {
    pub(crate) fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // player: Guid
        w.write_all(&self.player.guid().to_le_bytes())?;

        // amount_of_dungeons: u32
        w.write_all(&(self.dungeons.len() as u32).to_le_bytes())?;

        // dungeons: LfgJoinLockedDungeon[amount_of_dungeons]
        for i in self.dungeons.iter() {
            i.write_into_vec(w)?;
        }

        Ok(())
    }
}

impl LfgPartyInfo {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // player: Guid
        let player = Guid::read(r)?;

        // amount_of_dungeons: u32
        let amount_of_dungeons = crate::util::read_u32_le(r)?;

        // dungeons: LfgJoinLockedDungeon[amount_of_dungeons]
        let dungeons = {
            let mut dungeons = Vec::with_capacity(amount_of_dungeons as usize);
            for i in 0..amount_of_dungeons {
                dungeons.push(LfgJoinLockedDungeon::read(r)?);
            }
            dungeons
        };

        Ok(Self {
            player,
            dungeons,
        })
    }

}

impl LfgPartyInfo {
    pub(crate) fn size(&self) -> usize {
        8 // player: Guid
        + 4 // amount_of_dungeons: u32
        + self.dungeons.len() * 8 // dungeons: LfgJoinLockedDungeon[amount_of_dungeons]
    }
}

