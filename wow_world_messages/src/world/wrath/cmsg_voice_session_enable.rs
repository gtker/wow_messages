use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/cmsg_voice_session_enable.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/cmsg_voice_session_enable.wowm#L3):
/// ```text
/// cmsg CMSG_VOICE_SESSION_ENABLE = 0x03AF {
///     Bool isVoiceEnabled;
///     Bool isMicrophoneEnabled;
/// }
/// ```
pub struct CMSG_VOICE_SESSION_ENABLE {
    pub isVoiceEnabled: bool,
    pub isMicrophoneEnabled: bool,
}

impl crate::Message for CMSG_VOICE_SESSION_ENABLE {
    const OPCODE: u32 = 0x03af;

    fn size_without_header(&self) -> u32 {
        2
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // isVoiceEnabled: Bool
        w.write_all(u8::from(self.isVoiceEnabled).to_le_bytes().as_slice())?;

        // isMicrophoneEnabled: Bool
        w.write_all(u8::from(self.isMicrophoneEnabled).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 2 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03AF, size: body_size as u32 });
        }

        // isVoiceEnabled: Bool
        let isVoiceEnabled = crate::util::read_u8_le(r)? != 0;
        // isMicrophoneEnabled: Bool
        let isMicrophoneEnabled = crate::util::read_u8_le(r)? != 0;
        Ok(Self {
            isVoiceEnabled,
            isMicrophoneEnabled,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_VOICE_SESSION_ENABLE {}

