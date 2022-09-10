use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::tbc::MovementInfo;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/msg/msg_move_fall_land.wowm:80`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_fall_land.wowm#L80):
/// ```text
/// smsg MSG_MOVE_FALL_LAND_Server = 0x00C9 {
///     PackedGuid guid;
///     MovementInfo info;
/// }
/// ```
pub struct MSG_MOVE_FALL_LAND_Server {
    pub guid: Guid,
    pub info: MovementInfo,
}

impl crate::Message for MSG_MOVE_FALL_LAND_Server {
    const OPCODE: u32 = 0x00c9;

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
#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for MSG_MOVE_FALL_LAND_Server {}

impl MSG_MOVE_FALL_LAND_Server {
    pub(crate) fn size(&self) -> usize {
        self.guid.size() // guid: Guid
        + self.info.size() // info: MovementInfo
    }
}

