use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::wrath::MovementInfo;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/msg/msg_move_stop_swim.wowm:13`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_stop_swim.wowm#L13):
/// ```text
/// msg MSG_MOVE_STOP_SWIM = 0x00CB {
///     PackedGuid guid;
///     MovementInfo info;
/// }
/// ```
pub struct MSG_MOVE_STOP_SWIM {
    pub guid: Guid,
    pub info: MovementInfo,
}

impl crate::Message for MSG_MOVE_STOP_SWIM {
    const OPCODE: u32 = 0x00cb;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.write_packed_guid_into_vec(w);

        // info: MovementInfo
        self.info.write_into_vec(w)?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: PackedGuid
        let guid = Guid::read_packed(r)?;

        // info: MovementInfo
        let info = MovementInfo::read(r)?;

        Ok(Self {
            guid,
            info,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for MSG_MOVE_STOP_SWIM {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for MSG_MOVE_STOP_SWIM {}

impl MSG_MOVE_STOP_SWIM {
    pub(crate) fn size(&self) -> usize {
        self.guid.size() // guid: Guid
        + self.info.size() // info: MovementInfo
    }
}

