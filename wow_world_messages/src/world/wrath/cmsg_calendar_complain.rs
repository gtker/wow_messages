use std::io::{Read, Write};

use crate::Guid;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/cmsg_calendar_complain.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/cmsg_calendar_complain.wowm#L1):
/// ```text
/// cmsg CMSG_CALENDAR_COMPLAIN = 0x0446 {
///     Guid responsible_player;
///     Guid event;
///     Guid invite_id;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_CALENDAR_COMPLAIN {
    pub responsible_player: Guid,
    pub event: Guid,
    pub invite_id: Guid,
}

impl crate::private::Sealed for CMSG_CALENDAR_COMPLAIN {}
impl CMSG_CALENDAR_COMPLAIN {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 24 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // responsible_player: Guid
        let responsible_player = crate::util::read_guid(&mut r)?;

        // event: Guid
        let event = crate::util::read_guid(&mut r)?;

        // invite_id: Guid
        let invite_id = crate::util::read_guid(&mut r)?;

        Ok(Self {
            responsible_player,
            event,
            invite_id,
        })
    }

}

impl crate::Message for CMSG_CALENDAR_COMPLAIN {
    const OPCODE: u32 = 0x0446;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_CALENDAR_COMPLAIN"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_CALENDAR_COMPLAIN {{").unwrap();
        // Members
        writeln!(s, "    responsible_player = {};", self.responsible_player.guid()).unwrap();
        writeln!(s, "    event = {};", self.event.guid()).unwrap();
        writeln!(s, "    invite_id = {};", self.invite_id.guid()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 28_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1094_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "responsible_player", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "event", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "invite_id", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        24
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // responsible_player: Guid
        w.write_all(&self.responsible_player.guid().to_le_bytes())?;

        // event: Guid
        w.write_all(&self.event.guid().to_le_bytes())?;

        // invite_id: Guid
        w.write_all(&self.invite_id.guid().to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1094, "CMSG_CALENDAR_COMPLAIN", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_CALENDAR_COMPLAIN {}

