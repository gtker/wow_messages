use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Executes a query directly on the world server.
///
/// Not implemented on any major emulator.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/debug/cmsg_dblookup.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/debug/cmsg_dblookup.wowm#L1):
/// ```text
/// cmsg CMSG_DBLOOKUP = 0x0002 {
///     CString query;
/// }
/// ```
pub struct CMSG_DBLOOKUP {
    pub query: String,
}

impl crate::Message for CMSG_DBLOOKUP {
    const OPCODE: u32 = 0x0002;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // query: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.query.as_bytes().iter().rev().next(), Some(&0_u8), "String `query` must not be null-terminated.");
        w.write_all(self.query.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(1..=256).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0002, size: body_size as u32 });
        }

        // query: CString
        let query = crate::util::read_c_string_to_vec(r)?;
        let query = String::from_utf8(query)?;

        Ok(Self {
            query,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_DBLOOKUP {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_DBLOOKUP {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_DBLOOKUP {}

impl CMSG_DBLOOKUP {
    pub(crate) fn size(&self) -> usize {
        self.query.len() + 1 // query: CString
    }
}

