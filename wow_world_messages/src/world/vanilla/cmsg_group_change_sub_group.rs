use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/cmsg_group_change_sub_group.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_group_change_sub_group.wowm#L3):
/// ```text
/// cmsg CMSG_GROUP_CHANGE_SUB_GROUP = 0x027E {
///     CString name;
///     u8 group_number;
/// }
/// ```
pub struct CMSG_GROUP_CHANGE_SUB_GROUP {
    pub name: String,
    pub group_number: u8,
}

impl crate::Message for CMSG_GROUP_CHANGE_SUB_GROUP {
    const OPCODE: u32 = 0x027e;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.name.as_bytes().iter().rev().next(), Some(&0_u8), "String `name` must not be null-terminated.");
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // group_number: u8
        w.write_all(&self.group_number.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        // group_number: u8
        let group_number = crate::util::read_u8_le(r)?;

        Ok(Self {
            name,
            group_number,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_GROUP_CHANGE_SUB_GROUP {}

impl CMSG_GROUP_CHANGE_SUB_GROUP {
    pub(crate) fn size(&self) -> usize {
        self.name.len() + 1 // name: CString
        + 1 // group_number: u8
    }
}

