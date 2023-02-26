use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/cmsg_join_channel.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/cmsg_join_channel.wowm#L8):
/// ```text
/// cmsg CMSG_JOIN_CHANNEL = 0x0097 {
///     u32 channel_id;
///     u8 unknown1;
///     u8 unknown2;
///     CString channel_name;
///     CString channel_password;
/// }
/// ```
pub struct CMSG_JOIN_CHANNEL {
    pub channel_id: u32,
    pub unknown1: u8,
    pub unknown2: u8,
    pub channel_name: String,
    pub channel_password: String,
}

impl crate::Message for CMSG_JOIN_CHANNEL {
    const OPCODE: u32 = 0x0097;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // channel_id: u32
        w.write_all(&self.channel_id.to_le_bytes())?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: u8
        w.write_all(&self.unknown2.to_le_bytes())?;

        // channel_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.channel_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `channel_name` must not be null-terminated.");
        w.write_all(self.channel_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // channel_password: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.channel_password.as_bytes().iter().rev().next(), Some(&0_u8), "String `channel_password` must not be null-terminated.");
        w.write_all(self.channel_password.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(8..=518).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0097, size: body_size as u32 });
        }

        // channel_id: u32
        let channel_id = crate::util::read_u32_le(r)?;

        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(r)?;

        // unknown2: u8
        let unknown2 = crate::util::read_u8_le(r)?;

        // channel_name: CString
        let channel_name = {
            let channel_name = crate::util::read_c_string_to_vec(r)?;
            String::from_utf8(channel_name)?
        };

        // channel_password: CString
        let channel_password = {
            let channel_password = crate::util::read_c_string_to_vec(r)?;
            String::from_utf8(channel_password)?
        };

        Ok(Self {
            channel_id,
            unknown1,
            unknown2,
            channel_name,
            channel_password,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_JOIN_CHANNEL {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_JOIN_CHANNEL {}

impl CMSG_JOIN_CHANNEL {
    pub(crate) fn size(&self) -> usize {
        4 // channel_id: u32
        + 1 // unknown1: u8
        + 1 // unknown2: u8
        + self.channel_name.len() + 1 // channel_name: CString
        + self.channel_password.len() + 1 // channel_password: CString
    }
}

