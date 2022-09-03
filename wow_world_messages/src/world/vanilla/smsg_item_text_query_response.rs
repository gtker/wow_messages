use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_item_text_query_response.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_item_text_query_response.wowm#L3):
/// ```text
/// smsg SMSG_ITEM_TEXT_QUERY_RESPONSE = 0x0244 {
///     u32 item_text_id;
///     CString text;
/// }
/// ```
pub struct SMSG_ITEM_TEXT_QUERY_RESPONSE {
    pub item_text_id: u32,
    /// mangoszero: CString TODO: max length 8000
    ///
    pub text: String,
}

impl crate::Message for SMSG_ITEM_TEXT_QUERY_RESPONSE {
    const OPCODE: u32 = 0x0244;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // item_text_id: u32
        w.write_all(&self.item_text_id.to_le_bytes())?;

        // text: CString
        w.write_all(self.text.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // item_text_id: u32
        let item_text_id = crate::util::read_u32_le(r)?;

        // text: CString
        let text = crate::util::read_c_string_to_vec(r)?;
        let text = String::from_utf8(text)?;

        Ok(Self {
            item_text_id,
            text,
        })
    }

}
impl ServerMessage for SMSG_ITEM_TEXT_QUERY_RESPONSE {}

impl SMSG_ITEM_TEXT_QUERY_RESPONSE {
    pub(crate) fn size(&self) -> usize {
        4 // item_text_id: u32
        + self.text.len() + 1 // text: CString
    }
}

