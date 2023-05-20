use std::io::{Read, Write};

use crate::wrath::EquipmentSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_equipment_set_use.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_equipment_set_use.wowm#L9):
/// ```text
/// cmsg CMSG_EQUIPMENT_SET_USE = 0x04D5 {
///     EquipmentSet[19] sets;
/// }
/// ```
pub struct CMSG_EQUIPMENT_SET_USE {
    pub sets: [EquipmentSet; 19],
}

impl crate::private::Sealed for CMSG_EQUIPMENT_SET_USE {}
impl crate::Message for CMSG_EQUIPMENT_SET_USE {
    const OPCODE: u32 = 0x04d5;

    fn size_without_header(&self) -> u32 {
        190
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // sets: EquipmentSet[19]
        for i in self.sets.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 190 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04D5, size: body_size });
        }

        // sets: EquipmentSet[19]
        let sets = {
            let mut sets = [EquipmentSet::default(); 19];
            for i in sets.iter_mut() {
                *i = EquipmentSet::read(&mut r)?;
            }
            sets
        };

        Ok(Self {
            sets,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_EQUIPMENT_SET_USE {}

