use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
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
        let size_assert_header_size = w.len();
        // channel_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.channel_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `channel_name` must not be null-terminated.");
        w.write_all(self.channel_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
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
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_CHANNEL_ANNOUNCEMENTS {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_CHANNEL_ANNOUNCEMENTS {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_CHANNEL_ANNOUNCEMENTS {}

impl CMSG_CHANNEL_ANNOUNCEMENTS {
    pub(crate) fn size(&self) -> usize {
        self.channel_name.len() + 1 // channel_name: CString
    }
}

