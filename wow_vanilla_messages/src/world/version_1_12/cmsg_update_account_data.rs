use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// This is sent by the client after receiving [`SMSG_ACCOUNT_DATA_TIMES`](crate::world::version_1_12::SMSG_ACCOUNT_DATA_TIMES). Client can also request a block through [`CMSG_REQUEST_ACCOUNT_DATA`](crate::world::version_1_12::CMSG_REQUEST_ACCOUNT_DATA).
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/cmsg_update_account_data.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/cmsg_update_account_data.wowm#L3):
/// ```text
/// cmsg CMSG_UPDATE_ACCOUNT_DATA = 0x020B {
///     u32 block;
///     u32 unknown1;
///     u8[-] unknown2;
/// }
/// ```
pub struct CMSG_UPDATE_ACCOUNT_DATA {
    /// Exact meaning unknown. Seems to be between 0 and 7. Block 6 is changed when changing `layout-cache.txt` inside the WTF folder.
    ///
    pub block: u32,
    /// Seems to be equal to size of unknown2 for smaller messages. Perhaps uncompressed size?
    ///
    pub unknown1: u32,
    pub unknown2: Vec<u8>,
}

impl ClientMessage for CMSG_UPDATE_ACCOUNT_DATA {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // block: u32
        w.write_all(&self.block.to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: u8[-]
        for i in self.unknown2.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
    const OPCODE: u16 = 0x020b;

    fn client_size(&self) -> u16 {
        (self.size() + 6) as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // block: u32
        let block = crate::util::read_u32_le(r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        // unknown2: u8[-]
        let mut current_size = {
            4 // block: u32
            + 4 // unknown1: u32
        };
        let mut unknown2 = Vec::with_capacity(body_size as usize - current_size);
        while current_size < (body_size as usize) {
            unknown2.push(crate::util::read_u8_le(r)?);
            current_size += 1;
        }

        Ok(Self {
            block,
            unknown1,
            unknown2,
        })
    }

}

impl CMSG_UPDATE_ACCOUNT_DATA {
    pub(crate) fn size(&self) -> usize {
        4 // block: u32
        + 4 // unknown1: u32
        + self.unknown2.len() * core::mem::size_of::<u8>() // unknown2: u8[-]
    }
}

