use std::convert::{TryFrom, TryInto};
use crate::world::wrath::EquipmentSetListItem;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_equipment_set_list.wowm:10`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_equipment_set_list.wowm#L10):
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

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // amount_of_equipment_sets: u32
        w.write_all(&(self.equipment_sets.len() as u32).to_le_bytes())?;

        // equipment_sets: EquipmentSetListItem[amount_of_equipment_sets]
        for i in self.equipment_sets.iter() {
            i.write_into_vec(w)?;
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(4..=4294967294).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04BC, size: body_size as u32 });
        }

        // amount_of_equipment_sets: u32
        let amount_of_equipment_sets = crate::util::read_u32_le(r)?;

        // equipment_sets: EquipmentSetListItem[amount_of_equipment_sets]
        let mut equipment_sets = Vec::with_capacity(amount_of_equipment_sets as usize);
        for i in 0..amount_of_equipment_sets {
            equipment_sets.push(EquipmentSetListItem::read(r)?);
        }

        Ok(Self {
            equipment_sets,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_EQUIPMENT_SET_LIST {}

impl SMSG_EQUIPMENT_SET_LIST {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_equipment_sets: u32
        + self.equipment_sets.iter().fold(0, |acc, x| acc + x.size()) // equipment_sets: EquipmentSetListItem[amount_of_equipment_sets]
    }
}

