use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// This is sent by the client after receiving [`SMSG_ACCOUNT_DATA_TIMES`](crate::world::version_1_12::SMSG_ACCOUNT_DATA_TIMES). Layout and purpose unknown.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/cmsg_update_account_data.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/cmsg_update_account_data.wowm#L3):
/// ```text
/// cmsg CMSG_UPDATE_ACCOUNT_DATA = 0x020B {
///     u8[-] unknown;
/// }
/// ```
pub struct CMSG_UPDATE_ACCOUNT_DATA {
    pub unknown: Vec<u8>,
}

impl ClientMessage for CMSG_UPDATE_ACCOUNT_DATA {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // unknown: u8[-]
        for i in self.unknown.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
    const OPCODE: u16 = 0x020b;

    fn client_size(&self) -> u16 {
        (self.size() + 6) as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // unknown: u8[-]
        let mut current_size = {
            0
        };
        let mut unknown = Vec::with_capacity(body_size as usize - current_size);
        while current_size < (body_size as usize) {
            unknown.push(crate::util::read_u8_le(r)?);
            current_size += 1;
        }

        Ok(Self {
            unknown,
        })
    }

}

impl CMSG_UPDATE_ACCOUNT_DATA {
    pub(crate) fn size(&self) -> usize {
        self.unknown.len() * core::mem::size_of::<u8>() // unknown: u8[-]
    }
}

