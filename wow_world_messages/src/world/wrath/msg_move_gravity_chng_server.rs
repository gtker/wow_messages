use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::MovementInfo;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/msg/msg_move_gravity_chng.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_gravity_chng.wowm#L1):
/// ```text
/// smsg MSG_MOVE_GRAVITY_CHNG_Server = 0x04D2 {
///     PackedGuid player;
///     MovementInfo info;
/// }
/// ```
pub struct MSG_MOVE_GRAVITY_CHNG_Server {
    pub player: Guid,
    pub info: MovementInfo,
}

impl crate::Message for MSG_MOVE_GRAVITY_CHNG_Server {
    const OPCODE: u32 = 0x04d2;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // player: PackedGuid
        self.player.write_packed_guid_into_vec(&mut w)?;

        // info: MovementInfo
        self.info.write_into_vec(&mut w)?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(32..=93).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04D2, size: body_size as u32 });
        }

        // player: PackedGuid
        let player = Guid::read_packed(&mut r)?;

        // info: MovementInfo
        let info = MovementInfo::read(&mut r)?;

        Ok(Self {
            player,
            info,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_MOVE_GRAVITY_CHNG_Server {}

impl MSG_MOVE_GRAVITY_CHNG_Server {
    pub(crate) fn size(&self) -> usize {
        self.player.size() // player: PackedGuid
        + self.info.size() // info: MovementInfo
    }
}

