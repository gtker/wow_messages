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

impl crate::private::Sealed for SMSG_GUILD_EVENT {}
impl SMSG_GUILD_EVENT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(2..=65538).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x0092, size: body_size });
        }

        // event: GuildEvent
        let event = crate::util::read_u8_le(&mut r)?.try_into()?;

        // amount_of_events: u8
        let amount_of_events = crate::util::read_u8_le(&mut r)?;

        // event_descriptions: CString[amount_of_events]
        let event_descriptions = {
            let mut event_descriptions = Vec::with_capacity(amount_of_events as usize);
            for _ in 0..amount_of_events {
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

impl crate::Message for SMSG_GUILD_EVENT {
    const OPCODE: u32 = 0x0092;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_GUILD_EVENT {{").unwrap();
        // Members
        writeln!(s, "    event = {};", self.event.as_test_case_value()).unwrap();
        writeln!(s, "    amount_of_events = {};", self.event_descriptions.len()).unwrap();
        write!(s, "    event_descriptions = [").unwrap();
        for v in self.event_descriptions.as_slice() {
            write!(s, "\"{v}\", ").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 146_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "event", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "amount_of_events", "    ");
        if !self.event_descriptions.is_empty() {
            writeln!(s, "    /* event_descriptions: CString[amount_of_events] start */").unwrap();
            for (i, v) in self.event_descriptions.iter().enumerate() {
                crate::util::write_bytes(&mut s, &mut bytes, v.len() + 1, &format!("event_descriptions {i}"), "    ");
            }
            writeln!(s, "    /* event_descriptions: CString[amount_of_events] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // event: GuildEvent
        w.write_all(&(self.event.as_int().to_le_bytes()))?;

        // amount_of_events: u8
        w.write_all(&(self.event_descriptions.len() as u8).to_le_bytes())?;

        // event_descriptions: CString[amount_of_events]
        for i in self.event_descriptions.iter() {
            w.write_all(i.as_bytes())?;
            w.write_all(&[0])?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
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

