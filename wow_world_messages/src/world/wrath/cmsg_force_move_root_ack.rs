use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::wrath::MovementInfo;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_force_move_root_ack.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_force_move_root_ack.wowm#L9):
/// ```text
/// cmsg CMSG_FORCE_MOVE_ROOT_ACK = 0x00E9 {
///     PackedGuid guid;
///     u32 movement_counter;
///     MovementInfo info;
/// }
/// ```
pub struct CMSG_FORCE_MOVE_ROOT_ACK {
    pub guid: Guid,
    pub movement_counter: u32,
    pub info: MovementInfo,
}

impl crate::Message for CMSG_FORCE_MOVE_ROOT_ACK {
    const OPCODE: u32 = 0x00e9;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.write_packed_guid_into_vec(w);

        // movement_counter: u32
        w.write_all(&self.movement_counter.to_le_bytes())?;

        // info: MovementInfo
        self.info.write_into_vec(w)?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: PackedGuid
        let guid = Guid::read_packed(r)?;

        // movement_counter: u32
        let movement_counter = crate::util::read_u32_le(r)?;

        // info: MovementInfo
        let info = MovementInfo::read(r)?;

        Ok(Self {
            guid,
            movement_counter,
            info,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_FORCE_MOVE_ROOT_ACK {}

impl CMSG_FORCE_MOVE_ROOT_ACK {
    pub(crate) fn size(&self) -> usize {
        self.guid.size() // guid: Guid
        + 4 // movement_counter: u32
        + self.info.size() // info: MovementInfo
    }
}

