use crate:: {
};
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Sent in 3.3.5 by using the `bootme` console command. Command not available in 1.12. Available in 0.5.3.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/cmsg_bootme.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/cmsg_bootme.wowm#L1):
/// ```text
/// cmsg CMSG_BOOTME = 0x0001 {
/// }
/// ```
pub struct CMSG_BOOTME {
}

impl crate::Message for CMSG_BOOTME {
    const OPCODE: u32 = 0x0001;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0001, size: body_size as u32 });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_BOOTME {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_BOOTME {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_BOOTME {}

