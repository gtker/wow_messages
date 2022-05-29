use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::MovementInfo;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_MOVE_SPLINE_DONE {
    pub movement_info: MovementInfo,
    pub movement_counter: u32,
    pub unknown1: u32,
}

impl ClientMessage for CMSG_MOVE_SPLINE_DONE {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // movement_info: MovementInfo
        &self.movement_info.write_into_vec(w)?;;

        // movement_counter: u32
        w.write_all(&self.movement_counter.to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x02c9;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // movement_info: MovementInfo
        let movement_info = MovementInfo::read(r)?;

        // movement_counter: u32
        let movement_counter = crate::util::read_u32_le(r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        Ok(Self {
            movement_info,
            movement_counter,
            unknown1,
        })
    }

}

impl CMSG_MOVE_SPLINE_DONE {
    pub fn size(&self) -> usize {
        0
        + self.movement_info.size() // movement_info: MovementInfo
        + 4 // movement_counter: u32
        + 4 // unknown1: u32
    }
}

