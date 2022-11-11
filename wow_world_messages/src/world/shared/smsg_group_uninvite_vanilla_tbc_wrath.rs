use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_group_uninvite.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_group_uninvite.wowm#L3):
/// ```text
/// smsg SMSG_GROUP_UNINVITE = 0x0077 {
/// }
/// ```
pub struct SMSG_GROUP_UNINVITE {
}

impl crate::Message for SMSG_GROUP_UNINVITE {
    const OPCODE: u32 = 0x0077;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0077, size: body_size as u32 });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_GROUP_UNINVITE {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_GROUP_UNINVITE {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_GROUP_UNINVITE {}

