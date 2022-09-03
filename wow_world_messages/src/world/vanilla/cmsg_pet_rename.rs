use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/cmsg_pet_rename.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/cmsg_pet_rename.wowm#L3):
/// ```text
/// cmsg CMSG_PET_RENAME = 0x0177 {
///     Guid pet_guid;
///     CString name;
/// }
/// ```
pub struct CMSG_PET_RENAME {
    pub pet_guid: Guid,
    pub name: String,
}

impl crate::Message for CMSG_PET_RENAME {
    const OPCODE: u32 = 0x0177;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // pet_guid: Guid
        w.write_all(&self.pet_guid.guid().to_le_bytes())?;

        // name: CString
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // pet_guid: Guid
        let pet_guid = Guid::read(r)?;

        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        Ok(Self {
            pet_guid,
            name,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_PET_RENAME {}

impl CMSG_PET_RENAME {
    pub(crate) fn size(&self) -> usize {
        8 // pet_guid: Guid
        + self.name.len() + 1 // name: CString
    }
}

