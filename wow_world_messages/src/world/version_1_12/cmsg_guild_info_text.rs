use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/cmsg_guild_info_text.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/cmsg_guild_info_text.wowm#L3):
/// ```text
/// cmsg CMSG_GUILD_INFO_TEXT = 0x02FC {
///     CString guild_info;
/// }
/// ```
pub struct CMSG_GUILD_INFO_TEXT {
    pub guild_info: String,
}

impl ClientMessage for CMSG_GUILD_INFO_TEXT {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guild_info: CString
        w.write_all(self.guild_info.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    const OPCODE: u16 = 0x02fc;

    fn client_size(&self) -> u16 {
        (self.size() + 6) as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guild_info: CString
        let guild_info = crate::util::read_c_string_to_vec(r)?;
        let guild_info = String::from_utf8(guild_info)?;

        Ok(Self {
            guild_info,
        })
    }

}

impl CMSG_GUILD_INFO_TEXT {
    pub(crate) fn size(&self) -> usize {
        self.guild_info.len() + 1 // guild_info: CString
    }
}

