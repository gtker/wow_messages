use std::io::{Read, Write};

use crate::Guid;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/smsg_calendar_event_invite_removed.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/smsg_calendar_event_invite_removed.wowm#L1):
/// ```text
/// smsg SMSG_CALENDAR_EVENT_INVITE_REMOVED = 0x043B {
///     PackedGuid invitee;
///     Guid event_id;
///     u32 flags;
///     Bool show_alert;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_CALENDAR_EVENT_INVITE_REMOVED {
    pub invitee: Guid,
    pub event_id: Guid,
    pub flags: u32,
    pub show_alert: bool,
}

impl crate::private::Sealed for SMSG_CALENDAR_EVENT_INVITE_REMOVED {}
impl SMSG_CALENDAR_EVENT_INVITE_REMOVED {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(14..=22).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // invitee: PackedGuid
        let invitee = crate::util::read_packed_guid(&mut r)?;

        // event_id: Guid
        let event_id = crate::util::read_guid(&mut r)?;

        // flags: u32
        let flags = crate::util::read_u32_le(&mut r)?;

        // show_alert: Bool
        let show_alert = crate::util::read_bool_u8(&mut r)?;

        Ok(Self {
            invitee,
            event_id,
            flags,
            show_alert,
        })
    }

}

impl crate::Message for SMSG_CALENDAR_EVENT_INVITE_REMOVED {
    const OPCODE: u32 = 0x043b;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_CALENDAR_EVENT_INVITE_REMOVED"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_CALENDAR_EVENT_INVITE_REMOVED {{").unwrap();
        // Members
        writeln!(s, "    invitee = {};", self.invitee.guid()).unwrap();
        writeln!(s, "    event_id = {};", self.event_id.guid()).unwrap();
        writeln!(s, "    flags = {};", self.flags).unwrap();
        writeln!(s, "    show_alert = {};", if self.show_alert { "TRUE" } else { "FALSE" }).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1083_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.invitee), "invitee", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "event_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "flags", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "show_alert", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // invitee: PackedGuid
        crate::util::write_packed_guid(&self.invitee, &mut w)?;

        // event_id: Guid
        w.write_all(&self.event_id.guid().to_le_bytes())?;

        // flags: u32
        w.write_all(&self.flags.to_le_bytes())?;

        // show_alert: Bool
        w.write_all(u8::from(self.show_alert).to_le_bytes().as_slice())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1083, "SMSG_CALENDAR_EVENT_INVITE_REMOVED", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CALENDAR_EVENT_INVITE_REMOVED {}

impl SMSG_CALENDAR_EVENT_INVITE_REMOVED {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.invitee) // invitee: PackedGuid
        + 8 // event_id: Guid
        + 4 // flags: u32
        + 1 // show_alert: Bool
    }
}

