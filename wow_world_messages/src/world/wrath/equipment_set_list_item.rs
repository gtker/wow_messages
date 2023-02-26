use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_equipment_set_list.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_equipment_set_list.wowm#L1):
/// ```text
/// struct EquipmentSetListItem {
///     Guid guid;
///     CString name;
///     CString icon_name;
///     Guid[19] equipment;
/// }
/// ```
pub struct EquipmentSetListItem {
    pub guid: Guid,
    pub name: String,
    pub icon_name: String,
    pub equipment: [Guid; 19],
}

impl EquipmentSetListItem {
    pub(crate) fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.name.as_bytes().iter().rev().next(), Some(&0_u8), "String `name` must not be null-terminated.");
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // icon_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.icon_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `icon_name` must not be null-terminated.");
        w.write_all(self.icon_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // equipment: Guid[19]
        for i in self.equipment.iter() {
            w.write_all(&i.guid().to_le_bytes())?;
        }

        Ok(())
    }
}

impl EquipmentSetListItem {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // name: CString
        let name = {
            let name = crate::util::read_c_string_to_vec(r)?;
            String::from_utf8(name)?
        };

        // icon_name: CString
        let icon_name = {
            let icon_name = crate::util::read_c_string_to_vec(r)?;
            String::from_utf8(icon_name)?
        };

        // equipment: Guid[19]
        let equipment = {
            let mut equipment = [Guid::default(); 19];
            for i in equipment.iter_mut() {
                *i = Guid::read(r)?;
            }
            equipment
        };

        Ok(Self {
            guid,
            name,
            icon_name,
            equipment,
        })
    }

}

impl EquipmentSetListItem {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + self.name.len() + 1 // name: CString
        + self.icon_name.len() + 1 // icon_name: CString
        + self.equipment.iter().fold(0, |acc, _| acc + 8) // equipment: Guid[19]
    }
}

