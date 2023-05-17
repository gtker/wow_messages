use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/cmsg_group_accept.wowm:5`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_group_accept.wowm#L5):
/// ```text
/// cmsg CMSG_GROUP_ACCEPT = 0x0072 {
/// }
/// ```
pub struct CMSG_GROUP_ACCEPT {
}

impl crate::private::Sealed for CMSG_GROUP_ACCEPT {}
impl crate::Message for CMSG_GROUP_ACCEPT {
    const OPCODE: u32 = 0x0072;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0072, size: body_size });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_GROUP_ACCEPT {}

