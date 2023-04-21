use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_whois.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_whois.wowm#L3):
/// ```text
/// smsg SMSG_WHOIS = 0x0065 {
///     CString message;
/// }
/// ```
pub struct SMSG_WHOIS {
    /// vmangos: max CString length allowed: 256
    ///
    pub message: String,
}

impl crate::private::Sealed for SMSG_WHOIS {}
impl crate::Message for SMSG_WHOIS {
    const OPCODE: u32 = 0x0065;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // message: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.message.as_bytes().iter().rev().next(), Some(&0_u8), "String `message` must not be null-terminated.");
        w.write_all(self.message.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(1..=256).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0065, size: body_size as u32 });
        }

        // message: CString
        let message = {
            let message = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(message)?
        };

        Ok(Self {
            message,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_WHOIS {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_WHOIS {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_WHOIS {}

impl SMSG_WHOIS {
    pub(crate) fn size(&self) -> usize {
        self.message.len() + 1 // message: CString
    }
}

