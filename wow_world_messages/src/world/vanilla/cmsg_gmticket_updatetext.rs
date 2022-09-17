use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gamemaster/cmsg_gmticket_updatetext.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/cmsg_gmticket_updatetext.wowm#L3):
/// ```text
/// cmsg CMSG_GMTICKET_UPDATETEXT = 0x0207 {
///     CString message;
/// }
/// ```
pub struct CMSG_GMTICKET_UPDATETEXT {
    pub message: String,
}

impl crate::Message for CMSG_GMTICKET_UPDATETEXT {
    const OPCODE: u32 = 0x0207;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // message: CString
        // Guard against strings that are already null-terminated
        assert_ne!(self.message.as_bytes().iter().rev().next(), Some(&0u8), "String message must not be null-terminated.");
        w.write_all(self.message.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // message: CString
        let message = crate::util::read_c_string_to_vec(r)?;
        let message = String::from_utf8(message)?;

        Ok(Self {
            message,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_GMTICKET_UPDATETEXT {}

impl CMSG_GMTICKET_UPDATETEXT {
    pub(crate) fn size(&self) -> usize {
        self.message.len() + 1 // message: CString
    }
}

