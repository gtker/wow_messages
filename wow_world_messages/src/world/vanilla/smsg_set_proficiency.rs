use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::ItemClass;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_set_proficiency.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_set_proficiency.wowm#L3):
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

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // class: ItemClass
        w.write_all(&(self.class.as_int() as u8).to_le_bytes())?;

        // item_sub_class_mask: u32
        w.write_all(&self.item_sub_class_mask.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 5 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // class: ItemClass
        let class: ItemClass = crate::util::read_u8_le(r)?.try_into()?;

        // item_sub_class_mask: u32
        let item_sub_class_mask = crate::util::read_u32_le(r)?;

        Ok(Self {
            class,
            item_sub_class_mask,
        })
    }

}
#[cfg(any(feature = "vanilla", feature = "tbc"))]
impl crate::helper::shared::vanilla_tbc::ServerMessage for SMSG_SET_PROFICIENCY {}

