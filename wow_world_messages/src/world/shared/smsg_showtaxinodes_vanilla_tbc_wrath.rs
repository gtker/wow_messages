use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_showtaxinodes.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_showtaxinodes.wowm#L3):
/// ```text
/// smsg SMSG_SHOWTAXINODES = 0x01A9 {
///     u32 unknown1;
///     Guid guid;
///     u32 nearest_node;
///     u32[-] nodes;
/// }
/// ```
pub struct SMSG_SHOWTAXINODES {
    /// Set to 1 in mangoszero
    ///
    pub unknown1: u32,
    pub guid: Guid,
    pub nearest_node: u32,
    pub nodes: Vec<u32>,
}

impl crate::private::Sealed for SMSG_SHOWTAXINODES {}
impl crate::Message for SMSG_SHOWTAXINODES {
    const OPCODE: u32 = 0x01a9;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // nearest_node: u32
        w.write_all(&self.nearest_node.to_le_bytes())?;

        // nodes: u32[-]
        for i in self.nodes.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(16..=65551).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01A9, size: body_size });
        }

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(&mut r)?;

        // guid: Guid
        let guid = Guid::read(&mut r)?;

        // nearest_node: u32
        let nearest_node = crate::util::read_u32_le(&mut r)?;

        // nodes: u32[-]
        let nodes = {
            let mut current_size = {
                4 // unknown1: u32
                + 8 // guid: Guid
                + 4 // nearest_node: u32
            };
            let mut nodes = Vec::with_capacity(body_size as usize - current_size);
            while current_size < (body_size as usize) {
                nodes.push(crate::util::read_u32_le(&mut r)?);
                current_size += 1;
            }
            nodes
        };

        Ok(Self {
            unknown1,
            guid,
            nearest_node,
            nodes,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_SHOWTAXINODES {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_SHOWTAXINODES {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SHOWTAXINODES {}

impl SMSG_SHOWTAXINODES {
    pub(crate) fn size(&self) -> usize {
        4 // unknown1: u32
        + 8 // guid: Guid
        + 4 // nearest_node: u32
        + self.nodes.len() * core::mem::size_of::<u32>() // nodes: u32[-]
    }
}

