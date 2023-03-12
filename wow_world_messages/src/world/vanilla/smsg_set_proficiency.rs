use std::io::{Read, Write};
use crate::vanilla::ItemClass;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_set_proficiency.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_set_proficiency.wowm#L1):
/// ```text
/// smsg SMSG_SET_PROFICIENCY = 0x0127 {
///     ItemClass class;
///     u32 item_sub_class_mask;
/// }
/// ```
pub struct SMSG_SET_PROFICIENCY {
    pub class: ItemClass,
    pub item_sub_class_mask: u32,
}

impl crate::Message for SMSG_SET_PROFICIENCY {
    const OPCODE: u32 = 0x0127;

    fn size_without_header(&self) -> u32 {
        5
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // class: ItemClass
        w.write_all(&u8::from(self.class.as_int()).to_le_bytes())?;

        // item_sub_class_mask: u32
        w.write_all(&self.item_sub_class_mask.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 5 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0127, size: body_size as u32 });
        }

        // class: ItemClass
        let class: ItemClass = crate::util::read_u8_le(&mut r)?.try_into()?;

        // item_sub_class_mask: u32
        let item_sub_class_mask = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            class,
            item_sub_class_mask,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_SET_PROFICIENCY {}

