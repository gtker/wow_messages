use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/cmsg_add_ignore.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_add_ignore.wowm#L3):
/// ```text
/// cmsg CMSG_ADD_IGNORE = 0x006C {
///     CString ignore_name;
/// }
/// ```
pub struct CMSG_ADD_IGNORE {
    pub ignore_name: String,
}

impl crate::Message for CMSG_ADD_IGNORE {
    const OPCODE: u32 = 0x006c;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // ignore_name: CString
        w.write_all(self.ignore_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // ignore_name: CString
        let ignore_name = crate::util::read_c_string_to_vec(r)?;
        let ignore_name = String::from_utf8(ignore_name)?;

        Ok(Self {
            ignore_name,
        })
    }

}
impl ClientMessage for CMSG_ADD_IGNORE {}

impl CMSG_ADD_IGNORE {
    pub(crate) fn size(&self) -> usize {
        self.ignore_name.len() + 1 // ignore_name: CString
    }
}

