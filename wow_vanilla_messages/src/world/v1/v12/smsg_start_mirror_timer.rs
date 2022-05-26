use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::TimerType;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_START_MIRROR_TIMER {
    pub timer: TimerType,
    pub time_remaining: u32,
    pub duration: u32,
    pub scale: u32,
    pub is_frozen: u8,
    pub id: u32,
}

impl SMSG_START_MIRROR_TIMER {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 21], std::io::Error> {
        let mut array_w = [0u8; 21];
        let mut w = array_w.as_mut_slice();
        // timer: TimerType
        w.write_all(&(self.timer.as_int() as u32).to_le_bytes())?;

        // time_remaining: u32
        w.write_all(&self.time_remaining.to_le_bytes())?;

        // duration: u32
        w.write_all(&self.duration.to_le_bytes())?;

        // scale: u32
        w.write_all(&self.scale.to_le_bytes())?;

        // is_frozen: u8
        w.write_all(&self.is_frozen.to_le_bytes())?;

        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        Ok(array_w)
    }
}

impl ServerMessage for SMSG_START_MIRROR_TIMER {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // timer: TimerType
        w.write_all(&(self.timer.as_int() as u32).to_le_bytes())?;

        // time_remaining: u32
        w.write_all(&self.time_remaining.to_le_bytes())?;

        // duration: u32
        w.write_all(&self.duration.to_le_bytes())?;

        // scale: u32
        w.write_all(&self.scale.to_le_bytes())?;

        // is_frozen: u8
        w.write_all(&self.is_frozen.to_le_bytes())?;

        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x01d9;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        21
    }

    type Error = crate::errors::ParseError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // timer: TimerType
        let timer: TimerType = crate::util::read_u32_le(r)?.try_into()?;

        // time_remaining: u32
        let time_remaining = crate::util::read_u32_le(r)?;

        // duration: u32
        let duration = crate::util::read_u32_le(r)?;

        // scale: u32
        let scale = crate::util::read_u32_le(r)?;

        // is_frozen: u8
        let is_frozen = crate::util::read_u8_le(r)?;

        // id: u32
        let id = crate::util::read_u32_le(r)?;

        Ok(Self {
            timer,
            time_remaining,
            duration,
            scale,
            is_frozen,
            id,
        })
    }

}

