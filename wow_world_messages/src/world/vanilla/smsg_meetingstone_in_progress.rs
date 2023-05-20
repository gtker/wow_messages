use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/meetingstone/smsg_meetingstone_in_progress.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/meetingstone/smsg_meetingstone_in_progress.wowm#L3):
/// ```text
/// smsg SMSG_MEETINGSTONE_IN_PROGRESS = 0x0298 {
/// }
/// ```
pub struct SMSG_MEETINGSTONE_IN_PROGRESS {
}

impl crate::private::Sealed for SMSG_MEETINGSTONE_IN_PROGRESS {}
impl crate::Message for SMSG_MEETINGSTONE_IN_PROGRESS {
    const OPCODE: u32 = 0x0298;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0298, size: body_size });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_MEETINGSTONE_IN_PROGRESS {}

