use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/cinematic/cmsg_complete_cinematic.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/cinematic/cmsg_complete_cinematic.wowm#L3):
/// ```text
/// cmsg CMSG_COMPLETE_CINEMATIC = 0x00FC {
/// }
/// ```
pub struct CMSG_COMPLETE_CINEMATIC {
}

impl ClientMessage for CMSG_COMPLETE_CINEMATIC {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        Ok(())
    }
    const OPCODE: u16 = 0x00fc;

    fn client_size(&self) -> u16 {
        6
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        Ok(Self {
        })
    }

}

