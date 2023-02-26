use crate::Guid;
use crate::wrath::MovementInfo;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/msg/msg_move_hover.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_hover.wowm#L1):
/// ```text
/// msg MSG_MOVE_HOVER = 0x00F7 {
///     PackedGuid player;
///     MovementInfo info;
/// }
/// ```
pub struct MSG_MOVE_HOVER {
    pub player: Guid,
    pub info: MovementInfo,
}

impl crate::Message for MSG_MOVE_HOVER {
    const OPCODE: u32 = 0x00f7;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // player: PackedGuid
        self.player.write_packed_guid_into_vec(w)?;

        // info: MovementInfo
        self.info.write_into_vec(w)?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(32..=93).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00F7, size: body_size as u32 });
        }

        // player: PackedGuid
        let player = Guid::read_packed(r)?;

        // info: MovementInfo
        let info = MovementInfo::read(r)?;

        Ok(Self {
            player,
            info,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for MSG_MOVE_HOVER {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_MOVE_HOVER {}

impl MSG_MOVE_HOVER {
    pub(crate) fn size(&self) -> usize {
        self.player.size() // player: Guid
        + self.info.size() // info: MovementInfo
    }
}

