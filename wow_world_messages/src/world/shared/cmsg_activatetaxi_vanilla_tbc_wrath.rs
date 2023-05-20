use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_activatetaxi.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_activatetaxi.wowm#L3):
/// ```text
/// cmsg CMSG_ACTIVATETAXI = 0x01AD {
///     Guid guid;
///     u32 source_node;
///     u32 destination_node;
/// }
/// ```
pub struct CMSG_ACTIVATETAXI {
    pub guid: Guid,
    pub source_node: u32,
    pub destination_node: u32,
}

impl crate::private::Sealed for CMSG_ACTIVATETAXI {}
impl crate::Message for CMSG_ACTIVATETAXI {
    const OPCODE: u32 = 0x01ad;

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // source_node: u32
        w.write_all(&self.source_node.to_le_bytes())?;

        // destination_node: u32
        w.write_all(&self.destination_node.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01AD, size: body_size });
        }

        // guid: Guid
        let guid = Guid::read(&mut r)?;

        // source_node: u32
        let source_node = crate::util::read_u32_le(&mut r)?;

        // destination_node: u32
        let destination_node = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            guid,
            source_node,
            destination_node,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_ACTIVATETAXI {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_ACTIVATETAXI {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_ACTIVATETAXI {}

