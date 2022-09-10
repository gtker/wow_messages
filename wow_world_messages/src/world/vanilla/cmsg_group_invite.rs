use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/cmsg_group_invite.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_group_invite.wowm#L3):
/// ```text
/// cmsg CMSG_GROUP_INVITE = 0x006E {
///     CString name;
/// }
/// ```
pub struct CMSG_GROUP_INVITE {
    pub name: String,
}

impl crate::Message for CMSG_GROUP_INVITE {
    const OPCODE: u32 = 0x006e;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // name: CString
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        Ok(Self {
            name,
        })
    }

}
#[cfg(any(feature = "vanilla", feature = "tbc"))]
impl crate::helper::shared::vanilla_tbc::ClientMessage for CMSG_GROUP_INVITE {}

impl CMSG_GROUP_INVITE {
    pub(crate) fn size(&self) -> usize {
        self.name.len() + 1 // name: CString
    }
}

