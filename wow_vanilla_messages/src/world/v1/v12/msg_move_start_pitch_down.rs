use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::MovementInfo;
use crate::{ClientMessage, ServerMessage};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct MSG_MOVE_START_PITCH_DOWN {
    pub info: MovementInfo,
}

impl MSG_MOVE_START_PITCH_DOWN {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // info: MovementInfo
        w.write_all(&self.info.as_bytes()?)?;

        Ok(w)
    }
}

impl ClientMessage for MSG_MOVE_START_PITCH_DOWN {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // info: MovementInfo
        w.write_all(&self.info.as_bytes()?)?;

        Ok(())
    }
    const OPCODE: u16 = 0x00c0;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // info: MovementInfo
        let info = MovementInfo::read(r)?;

        Ok(Self {
            info,
        })
    }

}

impl ServerMessage for MSG_MOVE_START_PITCH_DOWN {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // info: MovementInfo
        w.write_all(&self.info.as_bytes()?)?;

        Ok(())
    }
    const OPCODE: u16 = 0x00c0;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // info: MovementInfo
        let info = MovementInfo::read(r)?;

        Ok(Self {
            info,
        })
    }

}

impl MSG_MOVE_START_PITCH_DOWN {
    pub fn size(&self) -> usize {
        0
        + self.info.size() // info: MovementInfo
    }
}

