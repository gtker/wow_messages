use crate:: {
};
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/cmsg_channel_set_owner.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/cmsg_channel_set_owner.wowm#L3):
/// ```text
/// cmsg CMSG_CHANNEL_SET_OWNER = 0x009D {
///     CString channel_name;
///     CString new_owner;
/// }
/// ```
pub struct CMSG_CHANNEL_SET_OWNER {
    pub channel_name: String,
    pub new_owner: String,
}

impl crate::Message for CMSG_CHANNEL_SET_OWNER {
    const OPCODE: u32 = 0x009d;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // channel_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.channel_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `channel_name` must not be null-terminated.");
        w.write_all(self.channel_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // new_owner: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.new_owner.as_bytes().iter().rev().next(), Some(&0_u8), "String `new_owner` must not be null-terminated.");
        w.write_all(self.new_owner.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(2..=512).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x009D, size: body_size as u32 });
        }

        // channel_name: CString
        let channel_name = {
            let channel_name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(channel_name)?
        };

        // new_owner: CString
        let new_owner = {
            let new_owner = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(new_owner)?
        };

        Ok(Self {
            channel_name,
            new_owner,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_CHANNEL_SET_OWNER {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_CHANNEL_SET_OWNER {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_CHANNEL_SET_OWNER {}

impl CMSG_CHANNEL_SET_OWNER {
    pub(crate) fn size(&self) -> usize {
        self.channel_name.len() + 1 // channel_name: CString
        + self.new_owner.len() + 1 // new_owner: CString
    }
}

