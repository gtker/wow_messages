use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::Map;
use crate::world::v1::v12::TransferAbortReason;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_TRANSFER_ABORTED {
    pub map: Map,
    pub reason: TransferAbortReason,
}

impl SMSG_TRANSFER_ABORTED {
    pub const PADDING_VALUE: u8 = 0x00;

}

impl SMSG_TRANSFER_ABORTED {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 6], std::io::Error> {
        let mut array_w = [0u8; 6];
        let mut w = array_w.as_mut_slice();
        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        // reason: TransferAbortReason
        w.write_all(&(self.reason.as_int() as u8).to_le_bytes())?;

        // padding: u8
        w.write_all(&Self::PADDING_VALUE.to_le_bytes())?;

        Ok(array_w)
    }
}

impl ServerMessage for SMSG_TRANSFER_ABORTED {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        // reason: TransferAbortReason
        w.write_all(&(self.reason.as_int() as u8).to_le_bytes())?;

        // padding: u8
        w.write_all(&Self::PADDING_VALUE.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0040;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        6
    }

    type Error = crate::errors::ParseError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // map: Map
        let map: Map = crate::util::read_u32_le(r)?.try_into()?;

        // reason: TransferAbortReason
        let reason: TransferAbortReason = crate::util::read_u8_le(r)?.try_into()?;

        // padding: u8
        let _padding = crate::util::read_u8_le(r)?;
        // padding is expected to always be 0 (0)

        Ok(Self {
            map,
            reason,
        })
    }

}

