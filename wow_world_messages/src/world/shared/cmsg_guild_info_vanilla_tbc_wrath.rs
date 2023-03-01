use crate:: {
};
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/cmsg_guild_info.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/cmsg_guild_info.wowm#L3):
/// ```text
/// cmsg CMSG_GUILD_INFO = 0x0087 {
/// }
/// ```
pub struct CMSG_GUILD_INFO {
}

impl crate::Message for CMSG_GUILD_INFO {
    const OPCODE: u32 = 0x0087;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0087, size: body_size as u32 });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_GUILD_INFO {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_GUILD_INFO {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_GUILD_INFO {}

