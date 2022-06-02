use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_activatetaxi.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_activatetaxi.wowm#L3):
/// ```text
/// cmsg CMSG_ACTIVATETAXI = 0x01AD {
///     Guid guid;
///     u32[2] nodes;
/// }
/// ```
pub struct CMSG_ACTIVATETAXI {
    pub guid: Guid,
    pub nodes: [u32; 2],
}

impl ClientMessage for CMSG_ACTIVATETAXI {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // nodes: u32[2]
        for i in self.nodes.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
    const OPCODE: u16 = 0x01ad;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        16
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // nodes: u32[2]
        let mut nodes = [u32::default(); 2];
        for i in nodes.iter_mut() {
            *i = crate::util::read_u32_le(r)?;
        }

        Ok(Self {
            guid,
            nodes,
        })
    }

}

