use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// cmangos/vmangos/mangoszero ignore the data in this.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/cmsg_request_account_data.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/cmsg_request_account_data.wowm#L3):
/// ```text
/// cmsg CMSG_REQUEST_ACCOUNT_DATA = 0x020A {
///     u32 block;
/// }
/// ```
pub struct CMSG_REQUEST_ACCOUNT_DATA {
    /// The exact purpose is unknown, but [`CMSG_UPDATE_ACCOUNT_DATA`](crate::world::version_1_12::CMSG_UPDATE_ACCOUNT_DATA) and [`SMSG_ACCOUNT_DATA_TIMES`](crate::world::version_1_12::SMSG_ACCOUNT_DATA_TIMES) possibly also operate on these blocks.
    ///
    pub block: u32,
}

impl ClientMessage for CMSG_REQUEST_ACCOUNT_DATA {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // block: u32
        w.write_all(&self.block.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x020a;

    fn client_size(&self) -> u16 {
        10
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // block: u32
        let block = crate::util::read_u32_le(r)?;

        Ok(Self {
            block,
        })
    }

}

