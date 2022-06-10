use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/meetingstone/cmsg_meetingstone_info.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/meetingstone/cmsg_meetingstone_info.wowm#L3):
/// ```text
/// cmsg CMSG_MEETINGSTONE_INFO = 0x0296 {
/// }
/// ```
pub struct CMSG_MEETINGSTONE_INFO {
}

impl ClientMessage for CMSG_MEETINGSTONE_INFO {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        Ok(())
    }
    const OPCODE: u16 = 0x0296;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        0
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        Ok(Self {
        })
    }

}

