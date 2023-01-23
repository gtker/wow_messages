use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/cmsg_buy_stable_slot.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/cmsg_buy_stable_slot.wowm#L3):
/// ```text
/// cmsg CMSG_BUY_STABLE_SLOT = 0x0272 {
///     Guid npc;
/// }
/// ```
pub struct CMSG_BUY_STABLE_SLOT {
    pub npc: Guid,
}

impl crate::Message for CMSG_BUY_STABLE_SLOT {
    const OPCODE: u32 = 0x0272;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // npc: Guid
        w.write_all(&self.npc.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0272, size: body_size as u32 });
        }

        // npc: Guid
        let npc = Guid::read(r)?;

        Ok(Self {
            npc,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_BUY_STABLE_SLOT {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_BUY_STABLE_SLOT {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_BUY_STABLE_SLOT {}

