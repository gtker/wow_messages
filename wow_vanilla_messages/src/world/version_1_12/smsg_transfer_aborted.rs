use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::map::{Map, map_try_from, map_as_int};
use crate::world::version_1_12::TransferAbortReason;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_transfer_aborted.wowm:11`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_transfer_aborted.wowm#L11):
/// ```text
/// smsg SMSG_TRANSFER_ABORTED = 0x0040 {
///     Map map;
///     TransferAbortReason reason;
///     u8 padding = 0;
/// }
/// ```
pub struct SMSG_TRANSFER_ABORTED {
    pub map: Map,
    pub reason: TransferAbortReason,
}

impl SMSG_TRANSFER_ABORTED {
    /// The field `padding` is constantly specified to be:
    ///
    /// | Format | Value |
    /// | ------ | ----- |
    /// | Decimal | `0` |
    /// | Hex | `0x00` |
    /// | Original | `0` |
    ///
    /// **This field is not in the Rust struct, but is written as this constant value.**
    pub const PADDING_VALUE: u8 = 0x00;

}

impl ServerMessage for SMSG_TRANSFER_ABORTED {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // map: Map
        w.write_all(&(map_as_int(&self.map) as u32).to_le_bytes())?;

        // reason: TransferAbortReason
        w.write_all(&(self.reason.as_int() as u8).to_le_bytes())?;

        // padding: u8
        w.write_all(&Self::PADDING_VALUE.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0040;

    fn server_size(&self) -> u16 {
        10
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 6 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // map: Map
        let map: Map = map_try_from(crate::util::read_u32_le(r)?)?;

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

