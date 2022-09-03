use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/cmsg_channel_announcements.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/cmsg_channel_announcements.wowm#L3):
/// ```text
/// cmsg CMSG_CHANNEL_ANNOUNCEMENTS = 0x00A7 {
///     CString channel_name;
/// }
/// ```
pub struct CMSG_CHANNEL_ANNOUNCEMENTS {
    pub channel_name: String,
}

impl crate::Message for CMSG_CHANNEL_ANNOUNCEMENTS {
    const OPCODE: u32 = 0x00a7;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // channel_name: CString
        w.write_all(self.channel_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // channel_name: CString
        let channel_name = crate::util::read_c_string_to_vec(r)?;
        let channel_name = String::from_utf8(channel_name)?;

        Ok(Self {
            channel_name,
        })
    }

}
impl ClientMessage for CMSG_CHANNEL_ANNOUNCEMENTS {}

impl CMSG_CHANNEL_ANNOUNCEMENTS {
    pub(crate) fn size(&self) -> usize {
        self.channel_name.len() + 1 // channel_name: CString
    }
}

