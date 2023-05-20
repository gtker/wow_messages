use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/msg_battleground_player_positions.wowm:5`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/msg_battleground_player_positions.wowm#L5):
/// ```text
/// struct BattlegroundPlayerPosition {
///     Guid player;
///     f32 position_x;
///     f32 position_y;
/// }
/// ```
pub struct BattlegroundPlayerPosition {
    pub player: Guid,
    pub position_x: f32,
    pub position_y: f32,
}

impl BattlegroundPlayerPosition {
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // player: Guid
        w.write_all(&self.player.guid().to_le_bytes())?;

        // position_x: f32
        w.write_all(&self.position_x.to_le_bytes())?;

        // position_y: f32
        w.write_all(&self.position_y.to_le_bytes())?;

        Ok(())
    }
}

impl BattlegroundPlayerPosition {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, std::io::Error> {
        // player: Guid
        let player = Guid::read(&mut r)?;

        // position_x: f32
        let position_x = crate::util::read_f32_le(&mut r)?;

        // position_y: f32
        let position_y = crate::util::read_f32_le(&mut r)?;

        Ok(Self {
            player,
            position_x,
            position_y,
        })
    }

}

