use std::io::{Read, Write};

use crate::wrath::EquipmentSetListItem;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_equipment_set_list.wowm:10`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_equipment_set_list.wowm#L10):
/// ```text
/// smsg SMSG_EQUIPMENT_SET_LIST = 0x04BC {
///     u32 amount_of_equipment_sets;
///     EquipmentSetListItem[amount_of_equipment_sets] equipment_sets;
/// }
/// ```
pub struct SMSG_EQUIPMENT_SET_LIST {
    pub equipment_sets: Vec<EquipmentSetListItem>,
}

impl crate::Message for SMSG_EQUIPMENT_SET_LIST {
    const OPCODE: u32 = 0x04bc;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // amount_of_equipment_sets: u32
        w.write_all(&(self.equipment_sets.len() as u32).to_le_bytes())?;

        // equipment_sets: EquipmentSetListItem[amount_of_equipment_sets]
        for i in self.equipment_sets.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(4..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04BC, size: body_size as u32 });
        }

        // amount_of_equipment_sets: u32
        let amount_of_equipment_sets = crate::util::read_u32_le(&mut r)?;

        // equipment_sets: EquipmentSetListItem[amount_of_equipment_sets]
        let equipment_sets = {
            let mut equipment_sets = Vec::with_capacity(amount_of_equipment_sets as usize);
            for i in 0..amount_of_equipment_sets {
                equipment_sets.push(EquipmentSetListItem::read(&mut r)?);
            }
            equipment_sets
        };

        Ok(Self {
            equipment_sets,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_EQUIPMENT_SET_LIST {}

impl SMSG_EQUIPMENT_SET_LIST {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_equipment_sets: u32
        + self.equipment_sets.iter().fold(0, |acc, x| acc + x.size()) // equipment_sets: EquipmentSetListItem[amount_of_equipment_sets]
    }
}

