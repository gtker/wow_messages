use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_calendar_event_invite_removed.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_calendar_event_invite_removed.wowm#L1):
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

impl crate::Message for SMSG_CALENDAR_EVENT_INVITE_REMOVED {
    const OPCODE: u32 = 0x043b;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // invitee: PackedGuid
        self.invitee.write_packed_guid_into_vec(w);

        // event_id: Guid
        w.write_all(&self.event_id.guid().to_le_bytes())?;

        // flags: u32
        w.write_all(&self.flags.to_le_bytes())?;

        // show_alert: Bool
        w.write_all(u8::from(self.show_alert).to_le_bytes().as_slice())?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(15..=22).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x043B, size: body_size as u32 });
        }

        // invitee: PackedGuid
        let invitee = Guid::read_packed(r)?;

        // event_id: Guid
        let event_id = Guid::read(r)?;

        // flags: u32
        let flags = crate::util::read_u32_le(r)?;

        // show_alert: Bool
        let show_alert = crate::util::read_u8_le(r)? != 0;
        Ok(Self {
            invitee,
            event_id,
            flags,
            show_alert,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_CALENDAR_EVENT_INVITE_REMOVED {}

impl SMSG_CALENDAR_EVENT_INVITE_REMOVED {
    pub(crate) fn size(&self) -> usize {
        self.invitee.size() // invitee: Guid
        + 8 // event_id: Guid
        + 4 // flags: u32
        + 1 // show_alert: Bool
    }
}

