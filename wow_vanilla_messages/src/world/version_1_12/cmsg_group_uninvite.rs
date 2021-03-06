use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/cmsg_group_uninvite.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_group_uninvite.wowm#L3):
/// ```text
/// cmsg CMSG_GROUP_UNINVITE = 0x0075 {
///     CString name;
/// }
/// ```
pub struct CMSG_GROUP_UNINVITE {
    pub name: String,
}

impl ClientMessage for CMSG_GROUP_UNINVITE {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // name: CString
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    const OPCODE: u16 = 0x0075;

    fn client_size(&self) -> u16 {
        (self.size() + 6) as u16
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

impl CMSG_GROUP_UNINVITE {
    pub(crate) fn size(&self) -> usize {
        self.name.len() + 1 // name: CString
    }
}

