use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_questupdate_add_item.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_questupdate_add_item.wowm#L3):
/// ```text
/// smsg SMSG_QUESTUPDATE_ADD_ITEM = 0x019A {
///     u32 required_item_id;
///     u32 items_required;
/// }
/// ```
pub struct SMSG_QUESTUPDATE_ADD_ITEM {
    pub required_item_id: u32,
    pub items_required: u32,
}

impl crate::Message for SMSG_QUESTUPDATE_ADD_ITEM {
    const OPCODE: u32 = 0x019a;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // required_item_id: u32
        w.write_all(&self.required_item_id.to_le_bytes())?;

        // items_required: u32
        w.write_all(&self.items_required.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // required_item_id: u32
        let required_item_id = crate::util::read_u32_le(r)?;

        // items_required: u32
        let items_required = crate::util::read_u32_le(r)?;

        Ok(Self {
            required_item_id,
            items_required,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_QUESTUPDATE_ADD_ITEM {}

