use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
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

impl crate::private::Sealed for SMSG_QUESTUPDATE_ADD_ITEM {}
impl crate::Message for SMSG_QUESTUPDATE_ADD_ITEM {
    const OPCODE: u32 = 0x019a;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // required_item_id: u32
        w.write_all(&self.required_item_id.to_le_bytes())?;

        // items_required: u32
        w.write_all(&self.items_required.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x019A, size: body_size as u32 });
        }

        // required_item_id: u32
        let required_item_id = crate::util::read_u32_le(&mut r)?;

        // items_required: u32
        let items_required = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            required_item_id,
            items_required,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_QUESTUPDATE_ADD_ITEM {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_QUESTUPDATE_ADD_ITEM {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_QUESTUPDATE_ADD_ITEM {}

