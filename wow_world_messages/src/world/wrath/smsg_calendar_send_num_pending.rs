use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Sent as response to [`CMSG_CALENDAR_GET_NUM_PENDING`](crate::wrath::CMSG_CALENDAR_GET_NUM_PENDING)
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/smsg_calendar_send_num_pending.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/smsg_calendar_send_num_pending.wowm#L3):
/// ```text
/// smsg SMSG_CALENDAR_SEND_NUM_PENDING = 0x0448 {
///     u32 pending_events;
/// }
/// ```
pub struct SMSG_CALENDAR_SEND_NUM_PENDING {
    /// Number of calendar items that require attention, e.g. pending invites
    ///
    pub pending_events: u32,
}

impl crate::Message for SMSG_CALENDAR_SEND_NUM_PENDING {
    const OPCODE: u32 = 0x0448;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // pending_events: u32
        w.write_all(&self.pending_events.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0448, size: body_size as u32 });
        }

        // pending_events: u32
        let pending_events = crate::util::read_u32_le(r)?;

        Ok(Self {
            pending_events,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CALENDAR_SEND_NUM_PENDING {}

