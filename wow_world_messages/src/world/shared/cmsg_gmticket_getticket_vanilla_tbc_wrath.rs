use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gamemaster/cmsg_gmticket_getticket.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/cmsg_gmticket_getticket.wowm#L3):
/// ```text
/// cmsg CMSG_GMTICKET_GETTICKET = 0x0211 {
/// }
/// ```
pub struct CMSG_GMTICKET_GETTICKET {
}

impl crate::private::Sealed for CMSG_GMTICKET_GETTICKET {}
impl crate::Message for CMSG_GMTICKET_GETTICKET {
    const OPCODE: u32 = 0x0211;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0211, size: body_size });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_GMTICKET_GETTICKET {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_GMTICKET_GETTICKET {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_GMTICKET_GETTICKET {}

