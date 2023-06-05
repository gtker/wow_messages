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

#[cfg(feature = "print-testcase")]
impl CMSG_VOICE_SESSION_ENABLE {
    pub fn to_test_case_string(&self) -> Option<String> {
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
        writeln!(s, "    versions = \"2.4.3 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for CMSG_VOICE_SESSION_ENABLE {}
impl crate::Message for CMSG_VOICE_SESSION_ENABLE {
    const OPCODE: u32 = 0x03af;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        CMSG_VOICE_SESSION_ENABLE::to_test_case_string(self)
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
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 2 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03AF, size: body_size });
        }

        // voice_enabled: Bool
        let voice_enabled = crate::util::read_u8_le(&mut r)? != 0;

        // microphone_enabled: Bool
        let microphone_enabled = crate::util::read_u8_le(&mut r)? != 0;

        Ok(Self {
            voice_enabled,
            microphone_enabled,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_VOICE_SESSION_ENABLE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_VOICE_SESSION_ENABLE {}

