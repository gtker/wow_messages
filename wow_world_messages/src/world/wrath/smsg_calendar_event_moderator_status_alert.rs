use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/smsg_calendar_event_moderator_status_alert.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/smsg_calendar_event_moderator_status_alert.wowm#L1):
/// ```text
/// smsg SMSG_CALENDAR_EVENT_MODERATOR_STATUS_ALERT = 0x0445 {
///     PackedGuid invitee;
///     Guid event_id;
///     u8 rank;
///     Bool show_alert;
/// }
/// ```
pub struct SMSG_CALENDAR_EVENT_MODERATOR_STATUS_ALERT {
    pub invitee: Guid,
    pub event_id: Guid,
    pub rank: u8,
    pub show_alert: bool,
}

impl crate::private::Sealed for SMSG_CALENDAR_EVENT_MODERATOR_STATUS_ALERT {}
impl crate::Message for SMSG_CALENDAR_EVENT_MODERATOR_STATUS_ALERT {
    const OPCODE: u32 = 0x0445;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // invitee: PackedGuid
        crate::util::write_packed_guid(&self.invitee, &mut w)?;

        // event_id: Guid
        w.write_all(&self.event_id.guid().to_le_bytes())?;

        // rank: u8
        w.write_all(&self.rank.to_le_bytes())?;

        // show_alert: Bool
        w.write_all(u8::from(self.show_alert).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(12..=19).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0445, size: body_size });
        }

        // invitee: PackedGuid
        let invitee = crate::util::read_packed_guid(&mut r)?;

        // event_id: Guid
        let event_id = crate::util::read_guid(&mut r)?;

        // rank: u8
        let rank = crate::util::read_u8_le(&mut r)?;

        // show_alert: Bool
        let show_alert = crate::util::read_u8_le(&mut r)? != 0;

        Ok(Self {
            invitee,
            event_id,
            rank,
            show_alert,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CALENDAR_EVENT_MODERATOR_STATUS_ALERT {}

impl SMSG_CALENDAR_EVENT_MODERATOR_STATUS_ALERT {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.invitee) // invitee: PackedGuid
        + 8 // event_id: Guid
        + 1 // rank: u8
        + 1 // show_alert: Bool
    }
}

