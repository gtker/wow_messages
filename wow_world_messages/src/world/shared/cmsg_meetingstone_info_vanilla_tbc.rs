use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/meetingstone/cmsg_meetingstone_info.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/meetingstone/cmsg_meetingstone_info.wowm#L3):
/// ```text
/// cmsg CMSG_MEETINGSTONE_INFO = 0x0296 {
/// }
/// ```
pub struct CMSG_MEETINGSTONE_INFO {
}

impl crate::Message for CMSG_MEETINGSTONE_INFO {
    const OPCODE: u32 = 0x0296;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0296, size: body_size as u32 });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_MEETINGSTONE_INFO {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_MEETINGSTONE_INFO {}

