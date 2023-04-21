use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/cmsg_channel_voice_on.wowm:5`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/cmsg_channel_voice_on.wowm#L5):
/// ```text
/// cmsg CMSG_CHANNEL_VOICE_ON = 0x03D6 {
/// }
/// ```
pub struct CMSG_CHANNEL_VOICE_ON {
}

impl crate::private::Sealed for CMSG_CHANNEL_VOICE_ON {}
impl crate::Message for CMSG_CHANNEL_VOICE_ON {
    const OPCODE: u32 = 0x03d6;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03D6, size: body_size as u32 });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_CHANNEL_VOICE_ON {}

