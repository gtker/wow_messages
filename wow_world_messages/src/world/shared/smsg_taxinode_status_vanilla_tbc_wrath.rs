use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_taxinode_status.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_taxinode_status.wowm#L3):
/// ```text
/// smsg SMSG_TAXINODE_STATUS = 0x01AB {
///     Guid guid;
///     Bool taxi_mask_node_known;
/// }
/// ```
pub struct SMSG_TAXINODE_STATUS {
    pub guid: Guid,
    pub taxi_mask_node_known: bool,
}

impl crate::Message for SMSG_TAXINODE_STATUS {
    const OPCODE: u32 = 0x01ab;

    fn size_without_header(&self) -> u32 {
        9
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // taxi_mask_node_known: Bool
        w.write_all(u8::from(self.taxi_mask_node_known).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 9 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01AB, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        // taxi_mask_node_known: Bool
        let taxi_mask_node_known = crate::util::read_u8_le(r)? != 0;

        Ok(Self {
            guid,
            taxi_mask_node_known,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_TAXINODE_STATUS {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_TAXINODE_STATUS {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_TAXINODE_STATUS {}

