use std::convert::{TryFrom, TryInto};
use crate::world::helper::Guid;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/msg_battleground_player_positions_server.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/msg_battleground_player_positions_server.wowm#L3):
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

impl ReadableAndWritable for BattlegroundPlayerPosition {
    type Error = std::io::Error;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // player: Guid
        let player = Guid::read(r)?;

        // position_x: f32
        let position_x = crate::util::read_f32_le(r)?;
        // position_y: f32
        let position_y = crate::util::read_f32_le(r)?;
        Ok(Self {
            player,
            position_x,
            position_y,
        })
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // player: Guid
        self.player.write(w)?;

        // position_x: f32
        w.write_all(&self.position_x.to_le_bytes())?;

        // position_y: f32
        w.write_all(&self.position_y.to_le_bytes())?;

        Ok(())
    }

}

impl ConstantSized for BattlegroundPlayerPosition {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for BattlegroundPlayerPosition {
    fn maximum_possible_size() -> usize {
        8 // player: Guid
        + 4 // position_x: f32
        + 4 // position_y: f32
    }
}

