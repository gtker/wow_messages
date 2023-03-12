use std::io::{Read, Write};
use wow_world_base::shared::inventory_type_vanilla_tbc_wrath::InventoryType;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_item_name_query_response.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_item_name_query_response.wowm#L8):
/// ```text
/// smsg SMSG_ITEM_NAME_QUERY_RESPONSE = 0x02C5 {
///     u32 item;
///     CString item_name;
///     InventoryType inventory_type;
/// }
/// ```
pub struct SMSG_ITEM_NAME_QUERY_RESPONSE {
    pub item: u32,
    pub item_name: String,
    pub inventory_type: InventoryType,
}

impl crate::Message for SMSG_ITEM_NAME_QUERY_RESPONSE {
    const OPCODE: u32 = 0x02c5;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        // item_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.item_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `item_name` must not be null-terminated.");
        w.write_all(self.item_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // inventory_type: InventoryType
        w.write_all(&u8::from(self.inventory_type.as_int()).to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(6..=261).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02C5, size: body_size as u32 });
        }

        // item: u32
        let item = crate::util::read_u32_le(&mut r)?;

        // item_name: CString
        let item_name = {
            let item_name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(item_name)?
        };

        // inventory_type: InventoryType
        let inventory_type: InventoryType = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            item,
            item_name,
            inventory_type,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_ITEM_NAME_QUERY_RESPONSE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_ITEM_NAME_QUERY_RESPONSE {}

impl SMSG_ITEM_NAME_QUERY_RESPONSE {
    pub(crate) fn size(&self) -> usize {
        4 // item: u32
        + self.item_name.len() + 1 // item_name: CString
        + 1 // inventory_type: InventoryType
    }
}

