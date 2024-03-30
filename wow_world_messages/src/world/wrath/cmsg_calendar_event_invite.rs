use std::io::{Read, Write};

use crate::Guid;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/cmsg_calendar_event_invite.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/cmsg_calendar_event_invite.wowm#L1):
/// ```text
/// cmsg CMSG_CALENDAR_EVENT_INVITE = 0x0431 {
///     Guid event;
///     Guid invite_id;
///     CString name;
///     Bool pre_event;
///     Bool guild_event;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_CALENDAR_EVENT_INVITE {
    pub event: Guid,
    pub invite_id: Guid,
    pub name: String,
    pub pre_event: bool,
    pub guild_event: bool,
}

impl crate::private::Sealed for CMSG_CALENDAR_EVENT_INVITE {}
impl CMSG_CALENDAR_EVENT_INVITE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(19..=274).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // event: Guid
        let event = crate::util::read_guid(&mut r)?;

        // invite_id: Guid
        let invite_id = crate::util::read_guid(&mut r)?;

        // name: CString
        let name = {
            let name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(name)?
        };

        // pre_event: Bool
        let pre_event = crate::util::read_bool_u8(&mut r)?;

        // guild_event: Bool
        let guild_event = crate::util::read_bool_u8(&mut r)?;

        Ok(Self {
            event,
            invite_id,
            name,
            pre_event,
            guild_event,
        })
    }

}

impl crate::Message for CMSG_CALENDAR_EVENT_INVITE {
    const OPCODE: u32 = 0x0431;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_CALENDAR_EVENT_INVITE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_CALENDAR_EVENT_INVITE {{").unwrap();
        // Members
        writeln!(s, "    event = {};", self.event.guid()).unwrap();
        writeln!(s, "    invite_id = {};", self.invite_id.guid()).unwrap();
        writeln!(s, "    name = \"{}\";", self.name).unwrap();
        writeln!(s, "    pre_event = {};", if self.pre_event { "TRUE" } else { "FALSE" }).unwrap();
        writeln!(s, "    guild_event = {};", if self.guild_event { "TRUE" } else { "FALSE" }).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1073_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "event", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "invite_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.name.len() + 1, "name", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "pre_event", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "guild_event", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // event: Guid
        w.write_all(&self.event.guid().to_le_bytes())?;

        // invite_id: Guid
        w.write_all(&self.invite_id.guid().to_le_bytes())?;

        // name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.name.as_bytes().iter().next_back(), Some(&0_u8), "String `name` must not be null-terminated.");
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // pre_event: Bool
        w.write_all(u8::from(self.pre_event).to_le_bytes().as_slice())?;

        // guild_event: Bool
        w.write_all(u8::from(self.guild_event).to_le_bytes().as_slice())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1073, "CMSG_CALENDAR_EVENT_INVITE", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_CALENDAR_EVENT_INVITE {}

impl CMSG_CALENDAR_EVENT_INVITE {
    pub(crate) fn size(&self) -> usize {
        8 // event: Guid
        + 8 // invite_id: Guid
        + self.name.len() + 1 // name: CString
        + 1 // pre_event: Bool
        + 1 // guild_event: Bool
    }
}

