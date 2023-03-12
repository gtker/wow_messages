use std::io::{Read, Write};

use wow_world_base::shared::loot_slot_type_vanilla_tbc_wrath::LootSlotType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/smsg_loot_response.wowm:87`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/smsg_loot_response.wowm#L87):
/// ```text
/// struct LootItem {
///     u8 index;
///     u32 item;
///     LootSlotType ty;
/// }
/// ```
pub struct LootItem {
    pub index: u8,
    pub item: u32,
    pub ty: LootSlotType,
}

impl LootItem {
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // index: u8
        w.write_all(&self.index.to_le_bytes())?;

        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        // ty: LootSlotType
        w.write_all(&u8::from(self.ty.as_int()).to_le_bytes())?;

        Ok(())
    }
}

impl LootItem {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> std::result::Result<Self, crate::errors::ParseError> {
        // index: u8
        let index = crate::util::read_u8_le(&mut r)?;

        // item: u32
        let item = crate::util::read_u32_le(&mut r)?;

        // ty: LootSlotType
        let ty: LootSlotType = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            index,
            item,
            ty,
        })
    }

}

