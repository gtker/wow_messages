use crate:: {
};
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gamemaster/cmsg_gmticketsystem_toggle.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/cmsg_gmticketsystem_toggle.wowm#L1):
/// ```text
/// cmsg CMSG_GMTICKETSYSTEM_TOGGLE = 0x029A {
/// }
/// ```
pub struct CMSG_GMTICKETSYSTEM_TOGGLE {
}

impl crate::Message for CMSG_GMTICKETSYSTEM_TOGGLE {
    const OPCODE: u32 = 0x029a;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x029A, size: body_size as u32 });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_GMTICKETSYSTEM_TOGGLE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_GMTICKETSYSTEM_TOGGLE {}

