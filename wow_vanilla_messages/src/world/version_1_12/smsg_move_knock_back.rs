use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_MOVE_KNOCK_BACK {
    pub guid: Guid,
    pub movement_counter: u32,
    pub v_cos: f32,
    pub v_sin: f32,
    pub horizontal_speed: f32,
    pub vertical_speed: f32,
}

impl ServerMessage for SMSG_MOVE_KNOCK_BACK {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        w.write_all(&self.guid.packed_guid())?;

        // movement_counter: u32
        w.write_all(&self.movement_counter.to_le_bytes())?;

        // v_cos: f32
        w.write_all(&self.v_cos.to_le_bytes())?;

        // v_sin: f32
        w.write_all(&self.v_sin.to_le_bytes())?;

        // horizontal_speed: f32
        w.write_all(&self.horizontal_speed.to_le_bytes())?;

        // vertical_speed: f32
        w.write_all(&self.vertical_speed.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x00ef;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: PackedGuid
        let guid = Guid::read_packed(r)?;

        // movement_counter: u32
        let movement_counter = crate::util::read_u32_le(r)?;

        // v_cos: f32
        let v_cos = crate::util::read_f32_le(r)?;
        // v_sin: f32
        let v_sin = crate::util::read_f32_le(r)?;
        // horizontal_speed: f32
        let horizontal_speed = crate::util::read_f32_le(r)?;
        // vertical_speed: f32
        let vertical_speed = crate::util::read_f32_le(r)?;
        Ok(Self {
            guid,
            movement_counter,
            v_cos,
            v_sin,
            horizontal_speed,
            vertical_speed,
        })
    }

}

impl SMSG_MOVE_KNOCK_BACK {
    pub(crate) fn size(&self) -> usize {
        self.guid.size() // guid: Guid
        + 4 // movement_counter: u32
        + 4 // v_cos: f32
        + 4 // v_sin: f32
        + 4 // horizontal_speed: f32
        + 4 // vertical_speed: f32
    }
}

