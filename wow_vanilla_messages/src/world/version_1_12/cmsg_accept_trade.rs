use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/trade/cmsg_accept_trade.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/trade/cmsg_accept_trade.wowm#L3):
/// ```text
/// cmsg CMSG_ACCEPT_TRADE = 0x011A {
///     u32 unknown1;
/// }
/// ```
pub struct CMSG_ACCEPT_TRADE {
    /// Skipped in vmangos and set to 1 for bots
    ///
    pub unknown1: u32,
}

impl ClientMessage for CMSG_ACCEPT_TRADE {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x011a;

    fn client_size(&self) -> u16 {
        10
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        Ok(Self {
            unknown1,
        })
    }

}

