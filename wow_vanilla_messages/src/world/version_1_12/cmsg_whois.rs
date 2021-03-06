use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/cmsg_whois.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_whois.wowm#L3):
/// ```text
/// cmsg CMSG_WHOIS = 0x0064 {
///     CString character;
/// }
/// ```
pub struct CMSG_WHOIS {
    pub character: String,
}

impl ClientMessage for CMSG_WHOIS {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // character: CString
        w.write_all(self.character.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    const OPCODE: u16 = 0x0064;

    fn client_size(&self) -> u16 {
        (self.size() + 6) as u16
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

impl CMSG_WHOIS {
    pub(crate) fn size(&self) -> usize {
        self.character.len() + 1 // character: CString
    }
}

