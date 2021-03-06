use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::MeetingStoneFailure;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/meetingstone/smsg_meetingstone_joinfailed.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/meetingstone/smsg_meetingstone_joinfailed.wowm#L9):
/// ```text
/// smsg SMSG_MEETINGSTONE_JOINFAILED = 0x02BB {
///     MeetingStoneFailure reason;
/// }
/// ```
pub struct SMSG_MEETINGSTONE_JOINFAILED {
    pub reason: MeetingStoneFailure,
}

impl ServerMessage for SMSG_MEETINGSTONE_JOINFAILED {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // reason: MeetingStoneFailure
        w.write_all(&(self.reason.as_int() as u8).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x02bb;

    fn server_size(&self) -> u16 {
        5
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // reason: MeetingStoneFailure
        let reason: MeetingStoneFailure = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            reason,
        })
    }

}

