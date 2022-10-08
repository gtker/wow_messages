use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/cmsg_leave_channel.wowm:7`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/cmsg_leave_channel.wowm#L7):
/// ```text
/// cmsg CMSG_LEAVE_CHANNEL = 0x0098 {
///     u32 channel_id;
///     CString channel_name;
/// }
/// ```
pub struct CMSG_LEAVE_CHANNEL {
    pub channel_id: u32,
    pub channel_name: String,
}

impl crate::Message for CMSG_LEAVE_CHANNEL {
    const OPCODE: u32 = 0x0098;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // channel_id: u32
        w.write_all(&self.channel_id.to_le_bytes())?;

        // channel_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.channel_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `channel_name` must not be null-terminated.");
        w.write_all(self.channel_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // channel_id: u32
        let channel_id = crate::util::read_u32_le(r)?;

        // channel_name: CString
        let channel_name = crate::util::read_c_string_to_vec(r)?;
        let channel_name = String::from_utf8(channel_name)?;

        Ok(Self {
            channel_id,
            channel_name,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_LEAVE_CHANNEL {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_LEAVE_CHANNEL {}

impl CMSG_LEAVE_CHANNEL {
    pub(crate) fn size(&self) -> usize {
        4 // channel_id: u32
        + self.channel_name.len() + 1 // channel_name: CString
    }
}

