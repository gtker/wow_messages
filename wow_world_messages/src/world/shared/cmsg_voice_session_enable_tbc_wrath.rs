use std::io::{Write, Read};

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

impl crate::Message for CMSG_VOICE_SESSION_ENABLE {
    const OPCODE: u32 = 0x03af;

    fn size_without_header(&self) -> u32 {
        2
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // voice_enabled: Bool
        w.write_all(u8::from(self.voice_enabled).to_le_bytes().as_slice())?;

        // microphone_enabled: Bool
        w.write_all(u8::from(self.microphone_enabled).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 2 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03AF, size: body_size as u32 });
        }

        // voice_enabled: Bool
        let voice_enabled = crate::util::read_u8_le(r)? != 0;

        // microphone_enabled: Bool
        let microphone_enabled = crate::util::read_u8_le(r)? != 0;

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

