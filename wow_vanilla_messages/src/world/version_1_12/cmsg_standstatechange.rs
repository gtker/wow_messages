use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::UnitStandState;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_STANDSTATECHANGE {
    pub animation_state: UnitStandState,
}

impl ClientMessage for CMSG_STANDSTATECHANGE {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // animation_state: UnitStandState
        w.write_all(&(self.animation_state.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0101;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        1
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // animation_state: UnitStandState
        let animation_state: UnitStandState = (crate::util::read_u32_le(r)? as u8).try_into()?;

        Ok(Self {
            animation_state,
        })
    }

}

