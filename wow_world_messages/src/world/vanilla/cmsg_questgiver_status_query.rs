use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
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

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            guid,
        })
    }

}
impl ClientMessage for CMSG_QUESTGIVER_STATUS_QUERY {}

