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

#[cfg(feature = "print-testcase")]
impl CMSG_WHOIS {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_WHOIS {{").unwrap();
        // Members
        writeln!(s, "    character = \"{}\";", self.character).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 100_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, self.character.len() + 1, "character", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for CMSG_WHOIS {}
impl crate::Message for CMSG_WHOIS {
    const OPCODE: u32 = 0x0064;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        CMSG_WHOIS::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // character: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.character.as_bytes().iter().rev().next(), Some(&0_u8), "String `character` must not be null-terminated.");
        w.write_all(self.character.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(1..=256).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0064, size: body_size });
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

