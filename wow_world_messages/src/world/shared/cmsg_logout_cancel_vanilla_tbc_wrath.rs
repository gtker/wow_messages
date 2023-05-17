use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/cmsg_logout_cancel.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/cmsg_logout_cancel.wowm#L3):
/// ```text
/// cmsg CMSG_LOGOUT_CANCEL = 0x004E {
/// }
/// ```
pub struct CMSG_LOGOUT_CANCEL {
}

impl crate::private::Sealed for CMSG_LOGOUT_CANCEL {}
impl crate::Message for CMSG_LOGOUT_CANCEL {
    const OPCODE: u32 = 0x004e;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x004E, size: body_size });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_LOGOUT_CANCEL {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_LOGOUT_CANCEL {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_LOGOUT_CANCEL {}

