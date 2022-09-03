use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/cmsg_guild_motd.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/cmsg_guild_motd.wowm#L3):
/// ```text
/// cmsg CMSG_GUILD_MOTD = 0x0091 {
///     CString message_of_the_day;
/// }
/// ```
pub struct CMSG_GUILD_MOTD {
    pub message_of_the_day: String,
}

impl crate::Message for CMSG_GUILD_MOTD {
    const OPCODE: u32 = 0x0091;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // message_of_the_day: CString
        w.write_all(self.message_of_the_day.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // message_of_the_day: CString
        let message_of_the_day = crate::util::read_c_string_to_vec(r)?;
        let message_of_the_day = String::from_utf8(message_of_the_day)?;

        Ok(Self {
            message_of_the_day,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_GUILD_MOTD {}

impl CMSG_GUILD_MOTD {
    pub(crate) fn size(&self) -> usize {
        self.message_of_the_day.len() + 1 // message_of_the_day: CString
    }
}

