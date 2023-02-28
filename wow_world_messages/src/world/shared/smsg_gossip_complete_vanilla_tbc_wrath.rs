use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gossip/smsg_gossip_complete.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gossip/smsg_gossip_complete.wowm#L3):
/// ```text
/// smsg SMSG_GOSSIP_COMPLETE = 0x017E {
/// }
/// ```
pub struct SMSG_GOSSIP_COMPLETE {
}

impl crate::Message for SMSG_GOSSIP_COMPLETE {
    const OPCODE: u32 = 0x017e;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x017E, size: body_size as u32 });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_GOSSIP_COMPLETE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_GOSSIP_COMPLETE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_GOSSIP_COMPLETE {}

