use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/cmsg_item_text_query.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/cmsg_item_text_query.wowm#L3):
/// ```text
/// cmsg CMSG_ITEM_TEXT_QUERY = 0x0243 {
///     u32 item_text_id;
///     u32 mail_id;
///     u32 unknown1;
/// }
/// ```
pub struct CMSG_ITEM_TEXT_QUERY {
    pub item_text_id: u32,
    /// # Comment
    ///
    /// vmangos/cmangos/mangoszero: this value can be item id in bag, but it is also mail id
    ///
    pub mail_id: u32,
    /// # Comment
    ///
    /// vmangos/cmangos/mangoszero: maybe something like state - 0x70000000
    ///
    pub unknown1: u32,
}

impl ClientMessage for CMSG_ITEM_TEXT_QUERY {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // item_text_id: u32
        w.write_all(&self.item_text_id.to_le_bytes())?;

        // mail_id: u32
        w.write_all(&self.mail_id.to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0243;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        12
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // item_text_id: u32
        let item_text_id = crate::util::read_u32_le(r)?;

        // mail_id: u32
        let mail_id = crate::util::read_u32_le(r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        Ok(Self {
            item_text_id,
            mail_id,
            unknown1,
        })
    }

}

