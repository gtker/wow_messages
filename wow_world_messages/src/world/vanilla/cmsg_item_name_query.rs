use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/cmsg_item_name_query.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/cmsg_item_name_query.wowm#L3):
/// ```text
/// cmsg CMSG_ITEM_NAME_QUERY = 0x02C4 {
///     u32 item_id;
///     Guid guid;
/// }
/// ```
pub struct CMSG_ITEM_NAME_QUERY {
    pub item_id: u32,
    pub guid: Guid,
}

impl crate::Message for CMSG_ITEM_NAME_QUERY {
    const OPCODE: u32 = 0x02c4;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // item_id: u32
        w.write_all(&self.item_id.to_le_bytes())?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // item_id: u32
        let item_id = crate::util::read_u32_le(r)?;

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            item_id,
            guid,
        })
    }

}
impl ClientMessage for CMSG_ITEM_NAME_QUERY {}

