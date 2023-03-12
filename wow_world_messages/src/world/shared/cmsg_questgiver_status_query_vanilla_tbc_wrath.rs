use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/cmsg_questgive_status_query.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/cmsg_questgive_status_query.wowm#L3):
/// ```text
/// cmsg CMSG_QUESTGIVER_STATUS_QUERY = 0x0182 {
///     Guid guid;
/// }
/// ```
pub struct CMSG_QUESTGIVER_STATUS_QUERY {
    pub guid: Guid,
}

impl crate::Message for CMSG_QUESTGIVER_STATUS_QUERY {
    const OPCODE: u32 = 0x0182;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0182, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(&mut r)?;

        Ok(Self {
            guid,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_QUESTGIVER_STATUS_QUERY {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_QUESTGIVER_STATUS_QUERY {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_QUESTGIVER_STATUS_QUERY {}

