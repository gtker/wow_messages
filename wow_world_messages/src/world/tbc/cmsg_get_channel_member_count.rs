use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/cmsg_get_channel_member_count.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/cmsg_get_channel_member_count.wowm#L1):
/// ```text
/// cmsg CMSG_GET_CHANNEL_MEMBER_COUNT = 0x03D3 {
///     CString channel;
/// }
/// ```
pub struct CMSG_GET_CHANNEL_MEMBER_COUNT {
    pub channel: String,
}

impl crate::Message for CMSG_GET_CHANNEL_MEMBER_COUNT {
    const OPCODE: u32 = 0x03d3;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // channel: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.channel.as_bytes().iter().rev().next(), Some(&0_u8), "String `channel` must not be null-terminated.");
        w.write_all(self.channel.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(1..=256).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03D3, size: body_size as u32 });
        }

        // channel: CString
        let channel = {
            let channel = crate::util::read_c_string_to_vec(r)?;
            String::from_utf8(channel)?
        };

        Ok(Self {
            channel,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_GET_CHANNEL_MEMBER_COUNT {}

impl CMSG_GET_CHANNEL_MEMBER_COUNT {
    pub(crate) fn size(&self) -> usize {
        self.channel.len() + 1 // channel: CString
    }
}

