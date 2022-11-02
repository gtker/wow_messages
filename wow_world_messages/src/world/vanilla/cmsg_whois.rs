use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/cmsg_whois.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_whois.wowm#L3):
/// ```text
/// cmsg CMSG_WHOIS = 0x0064 {
///     CString character;
/// }
/// ```
pub struct CMSG_WHOIS {
    pub character: String,
}

impl crate::Message for CMSG_WHOIS {
    const OPCODE: u32 = 0x0064;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // character: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.character.as_bytes().iter().rev().next(), Some(&0_u8), "String `character` must not be null-terminated.");
        w.write_all(self.character.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // character: CString
        let character = crate::util::read_c_string_to_vec(r)?;
        let character = String::from_utf8(character)?;

        Ok(Self {
            character,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_WHOIS {}

impl CMSG_WHOIS {
    pub(crate) fn size(&self) -> usize {
        self.character.len() + 1 // character: CString
    }
}

