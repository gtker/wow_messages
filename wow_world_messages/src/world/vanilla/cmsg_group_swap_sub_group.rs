use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/cmsg_group_swap_sub_group.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_group_swap_sub_group.wowm#L3):
/// ```text
/// cmsg CMSG_GROUP_SWAP_SUB_GROUP = 0x0280 {
///     CString name;
///     CString swap_with_name;
/// }
/// ```
pub struct CMSG_GROUP_SWAP_SUB_GROUP {
    pub name: String,
    pub swap_with_name: String,
}

impl crate::Message for CMSG_GROUP_SWAP_SUB_GROUP {
    const OPCODE: u32 = 0x0280;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // name: CString
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // swap_with_name: CString
        w.write_all(self.swap_with_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        // swap_with_name: CString
        let swap_with_name = crate::util::read_c_string_to_vec(r)?;
        let swap_with_name = String::from_utf8(swap_with_name)?;

        Ok(Self {
            name,
            swap_with_name,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_GROUP_SWAP_SUB_GROUP {}

impl CMSG_GROUP_SWAP_SUB_GROUP {
    pub(crate) fn size(&self) -> usize {
        self.name.len() + 1 // name: CString
        + self.swap_with_name.len() + 1 // swap_with_name: CString
    }
}

