use std::io::{Read, Write};

use crate::vanilla::GuildEvent;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/smsg_guild_event.wowm:66`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_guild_event.wowm#L66):
/// ```text
/// smsg SMSG_GUILD_EVENT = 0x0092 {
///     GuildEvent event;
///     u8 amount_of_events;
///     CString[amount_of_events] event_descriptions;
/// }
/// ```
pub struct SMSG_GUILD_EVENT {
    pub event: GuildEvent,
    pub event_descriptions: Vec<String>,
}

impl crate::Message for SMSG_GUILD_EVENT {
    const OPCODE: u32 = 0x0092;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // event: GuildEvent
        w.write_all(&u8::from(self.event.as_int()).to_le_bytes())?;

        // amount_of_events: u8
        w.write_all(&(self.event_descriptions.len() as u8).to_le_bytes())?;

        // event_descriptions: CString[amount_of_events]
        for i in self.event_descriptions.iter() {
            w.write_all(i.as_bytes())?;
            w.write_all(&[0])?;
        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(2..=65538).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0092, size: body_size as u32 });
        }

        // event: GuildEvent
        let event: GuildEvent = crate::util::read_u8_le(&mut r)?.try_into()?;

        // amount_of_events: u8
        let amount_of_events = crate::util::read_u8_le(&mut r)?;

        // event_descriptions: CString[amount_of_events]
        let event_descriptions = {
            let mut event_descriptions = Vec::with_capacity(amount_of_events as usize);
            for i in 0..amount_of_events {
                let s = crate::util::read_c_string_to_vec(&mut r)?;
                event_descriptions.push(String::from_utf8(s)?);
            }
            event_descriptions
        };

        Ok(Self {
            event,
            event_descriptions,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_GUILD_EVENT {}

impl SMSG_GUILD_EVENT {
    pub(crate) fn size(&self) -> usize {
        1 // event: GuildEvent
        + 1 // amount_of_events: u8
        + self.event_descriptions.iter().fold(0, |acc, x| acc + x.len() + 1) // event_descriptions: CString[amount_of_events]
    }
}

