use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Sent as response to [`CMSG_CALENDAR_GET_NUM_PENDING`](crate::wrath::CMSG_CALENDAR_GET_NUM_PENDING)
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/smsg_calendar_send_num_pending.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/smsg_calendar_send_num_pending.wowm#L3):
/// ```text
/// smsg SMSG_CALENDAR_SEND_NUM_PENDING = 0x0448 {
///     u32 pending_events;
/// }
/// ```
pub struct SMSG_CALENDAR_SEND_NUM_PENDING {
    /// Number of calendar items that require attention, e.g. pending invites
    pub pending_events: u32,
}

impl crate::private::Sealed for SMSG_CALENDAR_SEND_NUM_PENDING {}
impl crate::Message for SMSG_CALENDAR_SEND_NUM_PENDING {
    const OPCODE: u32 = 0x0448;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_CALENDAR_SEND_NUM_PENDING {{").unwrap();
        // Members
        writeln!(s, "    pending_events = {};", self.pending_events).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 6_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1096_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "pending_events", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // pending_events: u32
        w.write_all(&self.pending_events.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0448, size: body_size });
        }

        // pending_events: u32
        let pending_events = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            pending_events,
        })
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CALENDAR_SEND_NUM_PENDING {}

