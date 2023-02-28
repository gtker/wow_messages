use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/meetingstone/cmsg_meetingstone_leave.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/meetingstone/cmsg_meetingstone_leave.wowm#L3):
/// ```text
/// cmsg CMSG_MEETINGSTONE_LEAVE = 0x0293 {
/// }
/// ```
pub struct CMSG_MEETINGSTONE_LEAVE {
}

impl crate::Message for CMSG_MEETINGSTONE_LEAVE {
    const OPCODE: u32 = 0x0293;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0293, size: body_size as u32 });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_MEETINGSTONE_LEAVE {}

