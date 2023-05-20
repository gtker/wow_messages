use std::io::{Read, Write};

use crate::tbc::GuildLogEvent;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/msg_guild_event_log_query.wowm:27`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/msg_guild_event_log_query.wowm#L27):
/// ```text
/// smsg MSG_GUILD_EVENT_LOG_QUERY_Server = 0x03FE {
///     u8 amount_of_events;
///     GuildLogEvent[amount_of_events] events;
/// }
/// ```
pub struct MSG_GUILD_EVENT_LOG_QUERY_Server {
    pub events: Vec<GuildLogEvent>,
}

impl crate::private::Sealed for MSG_GUILD_EVENT_LOG_QUERY_Server {}
impl crate::Message for MSG_GUILD_EVENT_LOG_QUERY_Server {
    const OPCODE: u32 = 0x03fe;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // amount_of_events: u8
        w.write_all(&(self.events.len() as u8).to_le_bytes())?;

        // events: GuildLogEvent[amount_of_events]
        for i in self.events.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(1..=5377).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03FE, size: body_size });
        }

        // amount_of_events: u8
        let amount_of_events = crate::util::read_u8_le(&mut r)?;

        // events: GuildLogEvent[amount_of_events]
        let events = {
            let mut events = Vec::with_capacity(amount_of_events as usize);
            for _ in 0..amount_of_events {
                events.push(GuildLogEvent::read(&mut r)?);
            }
            events
        };

        Ok(Self {
            events,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for MSG_GUILD_EVENT_LOG_QUERY_Server {}

impl MSG_GUILD_EVENT_LOG_QUERY_Server {
    pub(crate) fn size(&self) -> usize {
        1 // amount_of_events: u8
        + self.events.iter().fold(0, |acc, x| acc + x.size()) // events: GuildLogEvent[amount_of_events]
    }
}

