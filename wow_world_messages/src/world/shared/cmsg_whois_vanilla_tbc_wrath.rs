use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
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

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // character: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.character.as_bytes().iter().rev().next(), Some(&0_u8), "String `character` must not be null-terminated.");
        w.write_all(self.character.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(1..=256).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0064, size: body_size as u32 });
        }

        // character: CString
        let character = {
            let character = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(character)?
        };

        Ok(Self {
            character,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_WHOIS {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_WHOIS {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_WHOIS {}

impl CMSG_WHOIS {
    pub(crate) fn size(&self) -> usize {
        self.character.len() + 1 // character: CString
    }
}

