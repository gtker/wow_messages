use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

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

impl crate::Message for CMSG_ACTIVATETAXI {
    const OPCODE: u32 = 0x01ad;

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // nodes: u32[2]
        for i in self.nodes.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

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
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_ACTIVATETAXI {}

