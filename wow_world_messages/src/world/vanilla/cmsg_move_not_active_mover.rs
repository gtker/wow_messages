use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::MovementInfo;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_move_not_active_mover.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_move_not_active_mover.wowm#L3):
/// ```text
/// cmsg CMSG_MOVE_NOT_ACTIVE_MOVER = 0x02D1 {
///     Guid old_mover;
///     MovementInfo info;
/// }
/// ```
pub struct CMSG_MOVE_NOT_ACTIVE_MOVER {
    pub old_mover: Guid,
    pub info: MovementInfo,
}

impl crate::Message for CMSG_MOVE_NOT_ACTIVE_MOVER {
    const OPCODE: u32 = 0x02d1;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // old_mover: Guid
        w.write_all(&self.old_mover.guid().to_le_bytes())?;

        // info: MovementInfo
        self.info.write_into_vec(w)?;

        assert_eq!(self.size() as usize, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // old_mover: Guid
        let old_mover = Guid::read(r)?;

        // info: MovementInfo
        let info = MovementInfo::read(r)?;

        Ok(Self {
            old_mover,
            info,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_MOVE_NOT_ACTIVE_MOVER {}

impl CMSG_MOVE_NOT_ACTIVE_MOVER {
    pub(crate) fn size(&self) -> usize {
        8 // old_mover: Guid
        + self.info.size() // info: MovementInfo
    }
}

