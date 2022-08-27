use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/meetingstone/smsg_meetingstone_in_progress.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/meetingstone/smsg_meetingstone_in_progress.wowm#L3):
/// ```text
/// smsg SMSG_MEETINGSTONE_IN_PROGRESS = 0x0298 {
/// }
/// ```
pub struct SMSG_MEETINGSTONE_IN_PROGRESS {
}

impl ServerMessage for SMSG_MEETINGSTONE_IN_PROGRESS {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        Ok(())
    }
    const OPCODE: u16 = 0x0298;

    fn server_size(&self) -> u16 {
        4
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        Ok(Self {
        })
    }

}

