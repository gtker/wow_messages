use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::ClientMessage;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/cmsg_guild_create.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/cmsg_guild_create.wowm#L3):
/// ```text
/// cmsg CMSG_GUILD_CREATE = 0x0081 {
///     CString guild_name;
/// }
/// ```
pub struct CMSG_GUILD_CREATE {
    pub guild_name: String,
}

impl crate::Message for CMSG_GUILD_CREATE {
    const OPCODE: u32 = 0x0081;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guild_name: CString
        w.write_all(self.guild_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guild_name: CString
        let guild_name = crate::util::read_c_string_to_vec(r)?;
        let guild_name = String::from_utf8(guild_name)?;

        Ok(Self {
            guild_name,
        })
    }

}
impl ClientMessage for CMSG_GUILD_CREATE {}

impl CMSG_GUILD_CREATE {
    pub(crate) fn size(&self) -> usize {
        self.guild_name.len() + 1 // guild_name: CString
    }
}

