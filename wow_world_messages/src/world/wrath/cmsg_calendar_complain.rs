use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/cmsg_calendar_complain.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/cmsg_calendar_complain.wowm#L1):
/// ```text
/// cmsg CMSG_CALENDAR_COMPLAIN = 0x0446 {
///     Guid responsible_player;
///     Guid event;
///     Guid invite_id;
/// }
/// ```
pub struct CMSG_CALENDAR_COMPLAIN {
    pub responsible_player: Guid,
    pub event: Guid,
    pub invite_id: Guid,
}

#[cfg(feature = "print-testcase")]
impl CMSG_CALENDAR_COMPLAIN {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_CALENDAR_COMPLAIN {{").unwrap();
        // Members
        writeln!(s, "    responsible_player = {};", self.responsible_player.guid()).unwrap();
        writeln!(s, "    event = {};", self.event.guid()).unwrap();
        writeln!(s, "    invite_id = {};", self.invite_id.guid()).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 30_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1094_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "responsible_player");
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

impl crate::private::Sealed for CMSG_CALENDAR_COMPLAIN {}
impl crate::Message for CMSG_CALENDAR_COMPLAIN {
    const OPCODE: u32 = 0x0446;

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
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 24 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0446, size: body_size });
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
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_CALENDAR_COMPLAIN {}

