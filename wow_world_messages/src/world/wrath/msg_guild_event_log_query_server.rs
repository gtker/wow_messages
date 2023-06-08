use std::io::{Read, Write};

use crate::wrath::GuildLogEvent;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/msg_guild_event_log_query.wowm:34`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/msg_guild_event_log_query.wowm#L34):
/// ```text
/// smsg MSG_GUILD_EVENT_LOG_QUERY_Server = 0x03FF {
///     u8 amount_of_events;
///     GuildLogEvent[amount_of_events] events;
/// }
/// ```
pub struct MSG_GUILD_EVENT_LOG_QUERY_Server {
    pub events: Vec<GuildLogEvent>,
}

#[cfg(feature = "print-testcase")]
impl MSG_GUILD_EVENT_LOG_QUERY_Server {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test MSG_GUILD_EVENT_LOG_QUERY_Server {{").unwrap();
        // Members
        writeln!(s, "    amount_of_events = {};", self.events.len()).unwrap();
        write!(s, "    events = [").unwrap();
        for v in self.events.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        event = {};", crate::tbc::GuildEvent::try_from(v.event.as_int()).unwrap().as_test_case_value()).unwrap();
            writeln!(s, "        player1 = {};", v.player1.guid()).unwrap();
            match &v.event {
                crate::tbc::GuildLogEvent_GuildEvent::Joined {
                    player2,
                } => {
                    writeln!(s, "        player2 = {};", player2.guid()).unwrap();
                }
                crate::tbc::GuildLogEvent_GuildEvent::Left {
                    player2,
                } => {
                    writeln!(s, "        player2 = {};", player2.guid()).unwrap();
                }
                _ => {}
            }

            writeln!(s, "        unix_time = {};", v.unix_time).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1023_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "amount_of_events", "    ");
        if !self.events.is_empty() {
            writeln!(s, "    /* events: GuildLogEvent[amount_of_events] start */").unwrap();
            for (i, v) in self.events.iter().enumerate() {
                writeln!(s, "    /* events: GuildLogEvent[amount_of_events] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 1, "event", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "player1", "        ");
                match &v.event {
                    crate::tbc::GuildLogEvent_GuildEvent::Joined {
                        player2,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 8, "player2", "        ");
                    }
                    crate::tbc::GuildLogEvent_GuildEvent::Left {
                        player2,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 8, "player2", "        ");
                    }
                    _ => {}
                }

                crate::util::write_bytes(&mut s, &mut bytes, 4, "unix_time", "        ");
                writeln!(s, "    /* events: GuildLogEvent[amount_of_events] {i} end */").unwrap();
            }
            writeln!(s, "    /* events: GuildLogEvent[amount_of_events] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for MSG_GUILD_EVENT_LOG_QUERY_Server {}
impl crate::Message for MSG_GUILD_EVENT_LOG_QUERY_Server {
    const OPCODE: u32 = 0x03ff;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        MSG_GUILD_EVENT_LOG_QUERY_Server::to_test_case_string(self)
    }

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
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03FF, size: body_size });
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

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_GUILD_EVENT_LOG_QUERY_Server {}

impl MSG_GUILD_EVENT_LOG_QUERY_Server {
    pub(crate) fn size(&self) -> usize {
        1 // amount_of_events: u8
        + self.events.iter().fold(0, |acc, x| acc + x.size()) // events: GuildLogEvent[amount_of_events]
    }
}

