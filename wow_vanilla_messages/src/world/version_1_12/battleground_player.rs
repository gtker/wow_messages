use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::version_1_12::PvpRank;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pvp/msg_pvp_log_data_server.wowm:14`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pvp/msg_pvp_log_data_server.wowm#L14):
/// ```text
/// struct BattlegroundPlayer {
///     Guid player;
///     PvpRank rank;
///     u32 killing_blows;
///     u32 honorable_kills;
///     u32 deaths;
///     u32 bonus_honor;
///     u32 amount_of_extra_fields;
///     u32[amount_of_extra_fields] fields;
/// }
/// ```
pub struct BattlegroundPlayer {
    pub player: Guid,
    pub rank: PvpRank,
    pub killing_blows: u32,
    pub honorable_kills: u32,
    pub deaths: u32,
    pub bonus_honor: u32,
    /// # Comment
    ///
    /// This depends on the BG in question. AV expects 7: Graveyards Assaulted, Graveyards Defended, Towers Assaulted, Towers Defended, Secondary Objectives, LieutenantCount, SecondaryNpc
    /// WSG expects 2: Flag captures and flag returns
    /// AB expects 2: Bases assaulted and bases defended
    pub fields: Vec<u32>,
}

impl BattlegroundPlayer {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // player: Guid
        w.write_all(&self.player.guid().to_le_bytes())?;

        // rank: PvpRank
        w.write_all(&(self.rank.as_int() as u32).to_le_bytes())?;

        // killing_blows: u32
        w.write_all(&self.killing_blows.to_le_bytes())?;

        // honorable_kills: u32
        w.write_all(&self.honorable_kills.to_le_bytes())?;

        // deaths: u32
        w.write_all(&self.deaths.to_le_bytes())?;

        // bonus_honor: u32
        w.write_all(&self.bonus_honor.to_le_bytes())?;

        // amount_of_extra_fields: u32
        w.write_all(&(self.fields.len() as u32).to_le_bytes())?;

        // fields: u32[amount_of_extra_fields]
        for i in self.fields.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
}

impl BattlegroundPlayer {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // player: Guid
        let player = Guid::read(r)?;

        // rank: PvpRank
        let rank: PvpRank = (crate::util::read_u32_le(r)? as u8).try_into()?;

        // killing_blows: u32
        let killing_blows = crate::util::read_u32_le(r)?;

        // honorable_kills: u32
        let honorable_kills = crate::util::read_u32_le(r)?;

        // deaths: u32
        let deaths = crate::util::read_u32_le(r)?;

        // bonus_honor: u32
        let bonus_honor = crate::util::read_u32_le(r)?;

        // amount_of_extra_fields: u32
        let amount_of_extra_fields = crate::util::read_u32_le(r)?;

        // fields: u32[amount_of_extra_fields]
        let mut fields = Vec::with_capacity(amount_of_extra_fields as usize);
        for i in 0..amount_of_extra_fields {
            fields.push(crate::util::read_u32_le(r)?);
        }

        Ok(Self {
            player,
            rank,
            killing_blows,
            honorable_kills,
            deaths,
            bonus_honor,
            fields,
        })
    }

}

impl BattlegroundPlayer {
    pub(crate) fn size(&self) -> usize {
        8 // player: Guid
        + 4 // rank: PvpRank
        + 4 // killing_blows: u32
        + 4 // honorable_kills: u32
        + 4 // deaths: u32
        + 4 // bonus_honor: u32
        + 4 // amount_of_extra_fields: u32
        + self.fields.len() * core::mem::size_of::<u32>() // fields: u32[amount_of_extra_fields]
    }
}

