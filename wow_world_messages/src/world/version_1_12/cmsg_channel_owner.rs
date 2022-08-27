use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/cmsg_channel_owner.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/cmsg_channel_owner.wowm#L3):
/// ```text
/// cmsg CMSG_CHANNEL_OWNER = 0x009E {
///     CString channel_name;
/// }
/// ```
pub struct CMSG_CHANNEL_OWNER {
    pub channel_name: String,
}

impl ClientMessage for CMSG_CHANNEL_OWNER {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // channel_name: CString
        w.write_all(self.channel_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    const OPCODE: u16 = 0x009e;

    fn client_size(&self) -> u16 {
        (self.size() + 6) as u16
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

impl CMSG_CHANNEL_OWNER {
    pub(crate) fn size(&self) -> usize {
        self.channel_name.len() + 1 // channel_name: CString
    }
}

