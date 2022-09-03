use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::ClientMessage;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/cmsg_cancel_auto_repeat_spell.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/cmsg_cancel_auto_repeat_spell.wowm#L3):
/// ```text
/// cmsg CMSG_CANCEL_AUTO_REPEAT_SPELL = 0x026D {
/// }
/// ```
pub struct CMSG_CANCEL_AUTO_REPEAT_SPELL {
}

impl crate::Message for CMSG_CANCEL_AUTO_REPEAT_SPELL {
    const OPCODE: u32 = 0x026d;

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
impl ClientMessage for CMSG_CANCEL_AUTO_REPEAT_SPELL {}

