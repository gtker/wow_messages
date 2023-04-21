use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/cmsg_pet_rename.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/cmsg_pet_rename.wowm#L1):
/// ```text
/// cmsg CMSG_PET_RENAME = 0x0177 {
///     Guid pet;
///     CString name;
/// }
/// ```
pub struct CMSG_PET_RENAME {
    pub pet: Guid,
    pub name: String,
}

impl crate::private::Sealed for CMSG_PET_RENAME {}
impl crate::Message for CMSG_PET_RENAME {
    const OPCODE: u32 = 0x0177;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // pet: Guid
        w.write_all(&self.pet.guid().to_le_bytes())?;

        // name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.name.as_bytes().iter().rev().next(), Some(&0_u8), "String `name` must not be null-terminated.");
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(9..=264).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0177, size: body_size as u32 });
        }

        // pet: Guid
        let pet = Guid::read(&mut r)?;

        // name: CString
        let name = {
            let name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(name)?
        };

        Ok(Self {
            pet,
            name,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_PET_RENAME {}

impl CMSG_PET_RENAME {
    pub(crate) fn size(&self) -> usize {
        8 // pet: Guid
        + self.name.len() + 1 // name: CString
    }
}

