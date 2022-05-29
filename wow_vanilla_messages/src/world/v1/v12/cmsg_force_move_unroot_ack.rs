use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::MovementInfo;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_FORCE_MOVE_UNROOT_ACK {
    pub guid: Guid,
    pub movement_counter: u32,
    pub movement_info: MovementInfo,
}

impl ClientMessage for CMSG_FORCE_MOVE_UNROOT_ACK {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // movement_counter: u32
        w.write_all(&self.movement_counter.to_le_bytes())?;

        // movement_info: MovementInfo
        &self.movement_info.write_into_vec(w)?;

        Ok(())
    }
    const OPCODE: u16 = 0x00eb;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // movement_counter: u32
        let movement_counter = crate::util::read_u32_le(r)?;

        // movement_info: MovementInfo
        let movement_info = MovementInfo::read(r)?;

        Ok(Self {
            guid,
            movement_counter,
            movement_info,
        })
    }

}

impl CMSG_FORCE_MOVE_UNROOT_ACK {
    pub(crate) fn size(&self) -> usize {
        0
        + 8 // guid: Guid
        + 4 // movement_counter: u32
        + self.movement_info.size() // movement_info: MovementInfo
    }
}

