use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/cmsg_set_active_voice_channel.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/cmsg_set_active_voice_channel.wowm#L8):
/// ```text
/// cmsg CMSG_SET_ACTIVE_VOICE_CHANNEL = 0x03D3 {
///     u32 unknown1;
///     CString unknown2;
/// }
/// ```
pub struct CMSG_SET_ACTIVE_VOICE_CHANNEL {
    pub unknown1: u32,
    pub unknown2: String,
}

impl crate::private::Sealed for CMSG_SET_ACTIVE_VOICE_CHANNEL {}
impl CMSG_SET_ACTIVE_VOICE_CHANNEL {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(5..=260).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(&mut r)?;

        // unknown2: CString
        let unknown2 = {
            let unknown2 = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(unknown2)?
        };

        Ok(Self {
            unknown1,
            unknown2,
        })
    }

}

impl crate::Message for CMSG_SET_ACTIVE_VOICE_CHANNEL {
    const OPCODE: u32 = 0x03d3;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_SET_ACTIVE_VOICE_CHANNEL"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_SET_ACTIVE_VOICE_CHANNEL {{").unwrap();
        // Members
        writeln!(s, "    unknown1 = {};", self.unknown1).unwrap();
        writeln!(s, "    unknown2 = \"{}\";", self.unknown2).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 979_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown1", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.unknown2.len() + 1, "unknown2", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.unknown2.as_bytes().iter().next_back(), Some(&0_u8), "String `unknown2` must not be null-terminated.");
        w.write_all(self.unknown2.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(979, "CMSG_SET_ACTIVE_VOICE_CHANNEL", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_SET_ACTIVE_VOICE_CHANNEL {}

impl CMSG_SET_ACTIVE_VOICE_CHANNEL {
    pub(crate) fn size(&self) -> usize {
        4 // unknown1: u32
        + self.unknown2.len() + 1 // unknown2: CString
    }
}

