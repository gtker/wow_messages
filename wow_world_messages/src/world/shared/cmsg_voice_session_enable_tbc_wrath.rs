use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/cmsg_voice_session_enable.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/cmsg_voice_session_enable.wowm#L3):
/// ```text
/// cmsg CMSG_VOICE_SESSION_ENABLE = 0x03AF {
///     Bool voice_enabled;
///     Bool microphone_enabled;
/// }
/// ```
pub struct CMSG_VOICE_SESSION_ENABLE {
    pub voice_enabled: bool,
    pub microphone_enabled: bool,
}

impl crate::private::Sealed for CMSG_VOICE_SESSION_ENABLE {}
impl CMSG_VOICE_SESSION_ENABLE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 2 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // voice_enabled: Bool
        let voice_enabled = crate::util::read_bool_u8(&mut r)?;

        // microphone_enabled: Bool
        let microphone_enabled = crate::util::read_bool_u8(&mut r)?;

        Ok(Self {
            voice_enabled,
            microphone_enabled,
        })
    }

}

impl crate::Message for CMSG_VOICE_SESSION_ENABLE {
    const OPCODE: u32 = 0x03af;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_VOICE_SESSION_ENABLE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_VOICE_SESSION_ENABLE {{").unwrap();
        // Members
        writeln!(s, "    voice_enabled = {};", if self.voice_enabled { "TRUE" } else { "FALSE" }).unwrap();
        writeln!(s, "    microphone_enabled = {};", if self.microphone_enabled { "TRUE" } else { "FALSE" }).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 6_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 943_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "voice_enabled", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "microphone_enabled", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        2
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // voice_enabled: Bool
        w.write_all(u8::from(self.voice_enabled).to_le_bytes().as_slice())?;

        // microphone_enabled: Bool
        w.write_all(u8::from(self.microphone_enabled).to_le_bytes().as_slice())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(943, "CMSG_VOICE_SESSION_ENABLE", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_VOICE_SESSION_ENABLE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_VOICE_SESSION_ENABLE {}

