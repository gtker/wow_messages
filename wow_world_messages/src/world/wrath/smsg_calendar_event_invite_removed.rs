use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/smsg_calendar_event_invite_removed.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/smsg_calendar_event_invite_removed.wowm#L1):
/// ```text
/// smsg SMSG_CALENDAR_EVENT_INVITE_REMOVED = 0x043B {
///     PackedGuid invitee;
///     Guid event_id;
///     u32 flags;
///     Bool show_alert;
/// }
/// ```
pub struct SMSG_CALENDAR_EVENT_INVITE_REMOVED {
    pub invitee: Guid,
    pub event_id: Guid,
    pub flags: u32,
    pub show_alert: bool,
}

#[cfg(feature = "print-testcase")]
impl SMSG_CALENDAR_EVENT_INVITE_REMOVED {
    pub fn to_test_case_string(&self) -> String {
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

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1083_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_CALENDAR_EVENT_INVITE_REMOVED {}
impl crate::Message for SMSG_CALENDAR_EVENT_INVITE_REMOVED {
    const OPCODE: u32 = 0x043b;

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
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(15..=22).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x043B, size: body_size });
        }

        // invitee: PackedGuid
        let invitee = crate::util::read_packed_guid(&mut r)?;

        // event_id: Guid
        let event_id = crate::util::read_guid(&mut r)?;

        // flags: u32
        let flags = crate::util::read_u32_le(&mut r)?;

        // show_alert: Bool
        let show_alert = crate::util::read_u8_le(&mut r)? != 0;

        Ok(Self {
            invitee,
            event_id,
            flags,
            show_alert,
        })
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

