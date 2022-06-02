use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/cmsg_add_friend.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_add_friend.wowm#L3):
/// ```text
/// cmsg CMSG_ADD_FRIEND = 0x0069 {
///     CString friend_name;
/// }
/// ```
pub struct CMSG_ADD_FRIEND {
    pub friend_name: String,
}

impl ClientMessage for CMSG_ADD_FRIEND {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // friend_name: CString
        w.write_all(self.friend_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    const OPCODE: u16 = 0x0069;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // friend_name: CString
        let friend_name = crate::util::read_c_string_to_vec(r)?;
        let friend_name = String::from_utf8(friend_name)?;

        Ok(Self {
            friend_name,
        })
    }

}

impl CMSG_ADD_FRIEND {
    pub(crate) fn size(&self) -> usize {
        self.friend_name.len() + 1 // friend_name: CString
    }
}

