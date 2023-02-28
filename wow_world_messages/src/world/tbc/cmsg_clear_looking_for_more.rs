use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/cmsg_clear_looking_for_more.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/cmsg_clear_looking_for_more.wowm#L1):
/// ```text
/// cmsg CMSG_CLEAR_LOOKING_FOR_MORE = 0x0364 {
/// }
/// ```
pub struct CMSG_CLEAR_LOOKING_FOR_MORE {
}

impl crate::Message for CMSG_CLEAR_LOOKING_FOR_MORE {
    const OPCODE: u32 = 0x0364;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0364, size: body_size as u32 });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_CLEAR_LOOKING_FOR_MORE {}

