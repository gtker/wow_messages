use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::MovementInfo;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
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

impl crate::private::Sealed for CMSG_MOVE_NOT_ACTIVE_MOVER {}
impl crate::Message for CMSG_MOVE_NOT_ACTIVE_MOVER {
    const OPCODE: u32 = 0x02d1;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // old_mover: Guid
        w.write_all(&self.old_mover.guid().to_le_bytes())?;

        // info: MovementInfo
        self.info.write_into_vec(&mut w)?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(36..=89).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02D1, size: body_size });
        }

        // old_mover: Guid
        let old_mover = Guid::read(&mut r)?;

        // info: MovementInfo
        let info = MovementInfo::read(&mut r)?;

        Ok(Self {
            old_mover,
            info,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_MOVE_NOT_ACTIVE_MOVER {}

impl CMSG_MOVE_NOT_ACTIVE_MOVER {
    pub(crate) const fn size(&self) -> usize {
        8 // old_mover: Guid
        + self.info.size() // info: MovementInfo
    }
}

