use crate:: {
};
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/msg_guild_event_log_query.wowm:5`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/msg_guild_event_log_query.wowm#L5):
/// ```text
/// cmsg MSG_GUILD_EVENT_LOG_QUERY_Client = 0x03FF {
/// }
/// ```
pub struct MSG_GUILD_EVENT_LOG_QUERY_Client {
}

impl crate::Message for MSG_GUILD_EVENT_LOG_QUERY_Client {
    const OPCODE: u32 = 0x03ff;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03FF, size: body_size as u32 });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for MSG_GUILD_EVENT_LOG_QUERY_Client {}

