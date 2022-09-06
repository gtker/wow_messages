use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/cmsg_join_channel.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/cmsg_join_channel.wowm#L3):
/// ```text
/// cmsg CMSG_JOIN_CHANNEL = 0x0097 {
///     CString channel_name;
///     CString channel_password;
/// }
/// ```
pub struct CMSG_JOIN_CHANNEL {
    pub channel_name: String,
    pub channel_password: String,
}

impl crate::Message for CMSG_JOIN_CHANNEL {
    const OPCODE: u32 = 0x0097;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // channel_name: CString
        w.write_all(self.channel_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // channel_password: CString
        w.write_all(self.channel_password.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // channel_name: CString
        let channel_name = crate::util::read_c_string_to_vec(r)?;
        let channel_name = String::from_utf8(channel_name)?;

        // channel_password: CString
        let channel_password = crate::util::read_c_string_to_vec(r)?;
        let channel_password = String::from_utf8(channel_password)?;

        Ok(Self {
            channel_name,
            channel_password,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_JOIN_CHANNEL {}

impl CMSG_JOIN_CHANNEL {
    pub(crate) fn size(&self) -> usize {
        self.channel_name.len() + 1 // channel_name: CString
        + self.channel_password.len() + 1 // channel_password: CString
    }
}

