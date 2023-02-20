use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/cmsg_group_uninvite.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_group_uninvite.wowm#L3):
/// ```text
/// cmsg CMSG_GROUP_UNINVITE = 0x0075 {
///     CString name;
/// }
/// ```
pub struct CMSG_GROUP_UNINVITE {
    pub name: String,
}

impl crate::Message for CMSG_GROUP_UNINVITE {
    const OPCODE: u32 = 0x0075;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.name.as_bytes().iter().rev().next(), Some(&0_u8), "String `name` must not be null-terminated.");
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(1..=256).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0075, size: body_size as u32 });
        }

        // name: CString
        let name = {
            let name = crate::util::read_c_string_to_vec(r)?;
            String::from_utf8(name)?
        };

        Ok(Self {
            name,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_GROUP_UNINVITE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_GROUP_UNINVITE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_GROUP_UNINVITE {}

impl CMSG_GROUP_UNINVITE {
    pub(crate) fn size(&self) -> usize {
        self.name.len() + 1 // name: CString
    }
}

