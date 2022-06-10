use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/cmsg_reset_instances.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/cmsg_reset_instances.wowm#L3):
/// ```text
/// cmsg CMSG_RESET_INSTANCES = 0x031D {
/// }
/// ```
pub struct CMSG_RESET_INSTANCES {
}

impl ClientMessage for CMSG_RESET_INSTANCES {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        Ok(())
    }
    const OPCODE: u16 = 0x031d;

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

