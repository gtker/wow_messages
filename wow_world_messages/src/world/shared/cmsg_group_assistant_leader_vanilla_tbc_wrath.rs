use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/cmsg_group_assistant_leader.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_group_assistant_leader.wowm#L3):
/// ```text
/// cmsg CMSG_GROUP_ASSISTANT_LEADER = 0x028F {
///     Guid guid;
///     Bool set_assistant;
/// }
/// ```
pub struct CMSG_GROUP_ASSISTANT_LEADER {
    pub guid: Guid,
    pub set_assistant: bool,
}

impl crate::Message for CMSG_GROUP_ASSISTANT_LEADER {
    const OPCODE: u32 = 0x028f;

    fn size_without_header(&self) -> u32 {
        9
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // set_assistant: Bool
        w.write_all(u8::from(self.set_assistant).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 9 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x028F, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        // set_assistant: Bool
        let set_assistant = crate::util::read_u8_le(r)? != 0;
        Ok(Self {
            guid,
            set_assistant,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_GROUP_ASSISTANT_LEADER {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_GROUP_ASSISTANT_LEADER {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_GROUP_ASSISTANT_LEADER {}

