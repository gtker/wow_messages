use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/cmsg_add_ignore.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_add_ignore.wowm#L3):
/// ```text
/// cmsg CMSG_ADD_IGNORE = 0x006C {
///     CString name;
/// }
/// ```
pub struct CMSG_ADD_IGNORE {
    pub name: String,
}

impl crate::private::Sealed for CMSG_ADD_IGNORE {}
impl crate::Message for CMSG_ADD_IGNORE {
    const OPCODE: u32 = 0x006c;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.name.as_bytes().iter().rev().next(), Some(&0_u8), "String `name` must not be null-terminated.");
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(1..=256).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x006C, size: body_size });
        }

        // name: CString
        let name = {
            let name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(name)?
        };

        Ok(Self {
            name,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_ADD_IGNORE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_ADD_IGNORE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_ADD_IGNORE {}

impl CMSG_ADD_IGNORE {
    pub(crate) fn size(&self) -> usize {
        self.name.len() + 1 // name: CString
    }
}

