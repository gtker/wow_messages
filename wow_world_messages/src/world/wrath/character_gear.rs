use std::io::{Read, Write};

use crate::wrath::InventoryType;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_char_enum_3_3_5.wowm:33`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_char_enum_3_3_5.wowm#L33):
/// ```text
/// struct CharacterGear {
///     u32 equipment_display_id;
///     InventoryType inventory_type;
///     u32 enchantment;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CharacterGear {
    pub equipment_display_id: u32,
    pub inventory_type: InventoryType,
    pub enchantment: u32,
}

impl CharacterGear {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // equipment_display_id: u32
        w.write_all(&self.equipment_display_id.to_le_bytes())?;

        // inventory_type: InventoryType
        w.write_all(&(self.inventory_type.as_int().to_le_bytes()))?;

        // enchantment: u32
        w.write_all(&self.enchantment.to_le_bytes())?;

        Ok(())
    }
}

impl CharacterGear {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // equipment_display_id: u32
        let equipment_display_id = crate::util::read_u32_le(&mut r)?;

        // inventory_type: InventoryType
        let inventory_type = crate::util::read_u8_le(&mut r)?.try_into()?;

        // enchantment: u32
        let enchantment = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            equipment_display_id,
            inventory_type,
            enchantment,
        })
    }

}

