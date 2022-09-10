use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_item_name_query_response.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_item_name_query_response.wowm#L3):
/// ```text
/// smsg SMSG_ITEM_NAME_QUERY_RESPONSE = 0x02C5 {
///     u32 item_id;
///     CString item_name;
/// }
/// ```
pub struct SMSG_ITEM_NAME_QUERY_RESPONSE {
    pub item_id: u32,
    pub item_name: String,
}

impl crate::Message for SMSG_ITEM_NAME_QUERY_RESPONSE {
    const OPCODE: u32 = 0x02c5;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // item_id: u32
        w.write_all(&self.item_id.to_le_bytes())?;

        // item_name: CString
        w.write_all(self.item_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // item_id: u32
        let item_id = crate::util::read_u32_le(r)?;

        // item_name: CString
        let item_name = crate::util::read_c_string_to_vec(r)?;
        let item_name = String::from_utf8(item_name)?;

        Ok(Self {
            item_id,
            item_name,
        })
    }

}
#[cfg(any(feature = "vanilla", feature = "tbc"))]
impl crate::helper::shared::vanilla_tbc::ServerMessage for SMSG_ITEM_NAME_QUERY_RESPONSE {}

impl SMSG_ITEM_NAME_QUERY_RESPONSE {
    pub(crate) fn size(&self) -> usize {
        4 // item_id: u32
        + self.item_name.len() + 1 // item_name: CString
    }
}

