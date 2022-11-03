use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_activatetaxiexpress.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_activatetaxiexpress.wowm#L1):
/// ```text
/// cmsg CMSG_ACTIVATETAXIEXPRESS = 0x0312 {
///     Guid guid;
///     u32 total_cost;
///     u32 node_count;
///     u32[node_count] nodes;
/// }
/// ```
pub struct CMSG_ACTIVATETAXIEXPRESS {
    pub guid: Guid,
    /// vmangos/mangosone: Never used.
    ///
    pub total_cost: u32,
    pub nodes: Vec<u32>,
}

impl crate::Message for CMSG_ACTIVATETAXIEXPRESS {
    const OPCODE: u32 = 0x0312;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // total_cost: u32
        w.write_all(&self.total_cost.to_le_bytes())?;

        // node_count: u32
        w.write_all(&(self.nodes.len() as u32).to_le_bytes())?;

        // nodes: u32[node_count]
        for i in self.nodes.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size < 16 || body_size > 4294967294 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0312, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        // total_cost: u32
        let total_cost = crate::util::read_u32_le(r)?;

        // node_count: u32
        let node_count = crate::util::read_u32_le(r)?;

        // nodes: u32[node_count]
        let mut nodes = Vec::with_capacity(node_count as usize);
        for i in 0..node_count {
            nodes.push(crate::util::read_u32_le(r)?);
        }

        Ok(Self {
            guid,
            total_cost,
            nodes,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_ACTIVATETAXIEXPRESS {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_ACTIVATETAXIEXPRESS {}

impl CMSG_ACTIVATETAXIEXPRESS {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // total_cost: u32
        + 4 // node_count: u32
        + self.nodes.len() * core::mem::size_of::<u32>() // nodes: u32[node_count]
    }
}

