use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/cmsg_questgiver_status_multiple_query.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/cmsg_questgiver_status_multiple_query.wowm#L1):
/// ```text
/// cmsg CMSG_QUESTGIVER_STATUS_MULTIPLE_QUERY = 0x0416 {
/// }
/// ```
pub struct CMSG_QUESTGIVER_STATUS_MULTIPLE_QUERY {
}

impl crate::private::Sealed for CMSG_QUESTGIVER_STATUS_MULTIPLE_QUERY {}
impl crate::Message for CMSG_QUESTGIVER_STATUS_MULTIPLE_QUERY {
    const OPCODE: u32 = 0x0416;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0416, size: body_size });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_QUESTGIVER_STATUS_MULTIPLE_QUERY {}

