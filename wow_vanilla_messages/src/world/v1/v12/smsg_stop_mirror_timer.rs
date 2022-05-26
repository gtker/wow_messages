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
pub struct SMSG_STOP_MIRROR_TIMER {
    pub timer: TimerType,
}

impl SMSG_STOP_MIRROR_TIMER {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 4], std::io::Error> {
        let mut array_w = [0u8; 4];
        let mut w = array_w.as_mut_slice();
        // timer: TimerType
        w.write_all(&(self.timer.as_int() as u32).to_le_bytes())?;

        Ok(array_w)
    }
}

impl ServerMessage for SMSG_STOP_MIRROR_TIMER {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // timer: TimerType
        w.write_all(&(self.timer.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x01db;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        4
    }

    type Error = crate::errors::ParseError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // timer: TimerType
        let timer: TimerType = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            timer,
        })
    }

}

