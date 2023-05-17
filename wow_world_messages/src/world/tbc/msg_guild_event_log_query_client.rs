use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/msg_guild_event_log_query.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/msg_guild_event_log_query.wowm#L1):
/// ```text
/// cmsg MSG_GUILD_EVENT_LOG_QUERY_Client = 0x03FE {
/// }
/// ```
pub struct MSG_GUILD_EVENT_LOG_QUERY_Client {
}

impl crate::private::Sealed for MSG_GUILD_EVENT_LOG_QUERY_Client {}
impl crate::Message for MSG_GUILD_EVENT_LOG_QUERY_Client {
    const OPCODE: u32 = 0x03fe;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03FE, size: body_size });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for MSG_GUILD_EVENT_LOG_QUERY_Client {}

