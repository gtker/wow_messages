use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/cmsg_guild_remove.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/cmsg_guild_remove.wowm#L3):
/// ```text
/// cmsg CMSG_GUILD_REMOVE = 0x008E {
///     CString player_name;
/// }
/// ```
pub struct CMSG_GUILD_REMOVE {
    pub player_name: String,
}

impl ClientMessage for CMSG_GUILD_REMOVE {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // player_name: CString
        w.write_all(self.player_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    const OPCODE: u16 = 0x008e;

    fn client_size(&self) -> u16 {
        (self.size() + 6) as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // player_name: CString
        let player_name = crate::util::read_c_string_to_vec(r)?;
        let player_name = String::from_utf8(player_name)?;

        Ok(Self {
            player_name,
        })
    }

}

impl CMSG_GUILD_REMOVE {
    pub(crate) fn size(&self) -> usize {
        self.player_name.len() + 1 // player_name: CString
    }
}

