use std::io::{Read, Write};

use crate::vanilla::MeetingStoneFailure;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/meetingstone/smsg_meetingstone_joinfailed.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/meetingstone/smsg_meetingstone_joinfailed.wowm#L9):
/// ```text
/// smsg SMSG_MEETINGSTONE_JOINFAILED = 0x02BB {
///     MeetingStoneFailure reason;
/// }
/// ```
pub struct SMSG_MEETINGSTONE_JOINFAILED {
    pub reason: MeetingStoneFailure,
}

impl crate::Message for SMSG_MEETINGSTONE_JOINFAILED {
    const OPCODE: u32 = 0x02bb;

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // reason: MeetingStoneFailure
        w.write_all(&u8::from(self.reason.as_int()).to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02BB, size: body_size as u32 });
        }

        // reason: MeetingStoneFailure
        let reason: MeetingStoneFailure = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            reason,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_MEETINGSTONE_JOINFAILED {}

