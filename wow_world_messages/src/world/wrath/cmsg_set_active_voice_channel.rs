use std::io::{Write, Read};

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

impl crate::Message for CMSG_SET_ACTIVE_VOICE_CHANNEL {
    const OPCODE: u32 = 0x03d3;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.unknown2.as_bytes().iter().rev().next(), Some(&0_u8), "String `unknown2` must not be null-terminated.");
        w.write_all(self.unknown2.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(5..=260).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03D3, size: body_size as u32 });
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
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_SET_ACTIVE_VOICE_CHANNEL {}

impl CMSG_SET_ACTIVE_VOICE_CHANNEL {
    pub(crate) fn size(&self) -> usize {
        4 // unknown1: u32
        + self.unknown2.len() + 1 // unknown2: CString
    }
}

