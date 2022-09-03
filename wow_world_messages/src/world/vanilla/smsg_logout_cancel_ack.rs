use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_logout_cancel_ack.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_logout_cancel_ack.wowm#L3):
/// ```text
/// smsg SMSG_LOGOUT_CANCEL_ACK = 0x004F {
/// }
/// ```
pub struct SMSG_LOGOUT_CANCEL_ACK {
}

impl crate::Message for SMSG_LOGOUT_CANCEL_ACK {
    const OPCODE: u32 = 0x004f;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        Ok(Self {
        })
    }

}
impl ServerMessage for SMSG_LOGOUT_CANCEL_ACK {}

