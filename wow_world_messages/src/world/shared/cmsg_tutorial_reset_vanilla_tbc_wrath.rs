use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/cmsg_tutorial_reset.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/cmsg_tutorial_reset.wowm#L3):
/// ```text
/// cmsg CMSG_TUTORIAL_RESET = 0x0100 {
/// }
/// ```
pub struct CMSG_TUTORIAL_RESET {
}

impl crate::Message for CMSG_TUTORIAL_RESET {
    const OPCODE: u32 = 0x0100;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0100, size: body_size as u32 });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_TUTORIAL_RESET {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_TUTORIAL_RESET {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_TUTORIAL_RESET {}

