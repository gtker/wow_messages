use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::version_1_12::MovementInfo;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_move_not_active_mover.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_move_not_active_mover.wowm#L3):
/// ```text
/// cmsg CMSG_MOVE_NOT_ACTIVE_MOVER = 0x02D1 {
///     Guid old_mover;
///     MovementInfo movement_info;
/// }
/// ```
pub struct CMSG_MOVE_NOT_ACTIVE_MOVER {
    pub old_mover: Guid,
    pub movement_info: MovementInfo,
}

impl ClientMessage for CMSG_MOVE_NOT_ACTIVE_MOVER {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // old_mover: Guid
        w.write_all(&self.old_mover.guid().to_le_bytes())?;

        // movement_info: MovementInfo
        self.movement_info.write_into_vec(w)?;

        Ok(())
    }
    const OPCODE: u16 = 0x02d1;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // old_mover: Guid
        let old_mover = Guid::read(r)?;

        // movement_info: MovementInfo
        let movement_info = MovementInfo::read(r)?;

        Ok(Self {
            old_mover,
            movement_info,
        })
    }

}

impl CMSG_MOVE_NOT_ACTIVE_MOVER {
    pub(crate) fn size(&self) -> usize {
        8 // old_mover: Guid
        + self.movement_info.size() // movement_info: MovementInfo
    }
}

