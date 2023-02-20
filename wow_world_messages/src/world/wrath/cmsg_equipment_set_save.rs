use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_equipment_set_save.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_equipment_set_save.wowm#L1):
/// ```text
/// cmsg CMSG_EQUIPMENT_SET_SAVE = 0x04BD {
///     PackedGuid guid;
///     u32 index;
///     CString name;
///     CString icon_name;
///     Guid[19] equipment;
/// }
/// ```
pub struct CMSG_EQUIPMENT_SET_SAVE {
    pub guid: Guid,
    pub index: u32,
    pub name: String,
    pub icon_name: String,
    pub equipment: [Guid; 19],
}

impl crate::Message for CMSG_EQUIPMENT_SET_SAVE {
    const OPCODE: u32 = 0x04bd;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // guid: PackedGuid
        self.guid.write_packed_guid_into_vec(w);

        // index: u32
        w.write_all(&self.index.to_le_bytes())?;

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

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(160..=677).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04BD, size: body_size as u32 });
        }

        // guid: PackedGuid
        let guid = Guid::read_packed(r)?;

        // index: u32
        let index = crate::util::read_u32_le(r)?;

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
            index,
            name,
            icon_name,
            equipment,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_EQUIPMENT_SET_SAVE {}

impl CMSG_EQUIPMENT_SET_SAVE {
    pub(crate) fn size(&self) -> usize {
        self.guid.size() // guid: Guid
        + 4 // index: u32
        + self.name.len() + 1 // name: CString
        + self.icon_name.len() + 1 // icon_name: CString
        + self.equipment.iter().fold(0, |acc, _| acc + 8) // equipment: Guid[19]
    }
}

