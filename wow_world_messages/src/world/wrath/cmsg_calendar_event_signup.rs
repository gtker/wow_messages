use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/cmsg_calendar_event_signup.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/cmsg_calendar_event_signup.wowm#L1):
/// ```text
/// cmsg CMSG_CALENDAR_EVENT_SIGNUP = 0x04BA {
///     Guid event_id;
///     Bool tentative;
/// }
/// ```
pub struct CMSG_CALENDAR_EVENT_SIGNUP {
    pub event_id: Guid,
    pub tentative: bool,
}

impl crate::Message for CMSG_CALENDAR_EVENT_SIGNUP {
    const OPCODE: u32 = 0x04ba;

    fn size_without_header(&self) -> u32 {
        9
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // event_id: Guid
        w.write_all(&self.event_id.guid().to_le_bytes())?;

        // tentative: Bool
        w.write_all(u8::from(self.tentative).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 9 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04BA, size: body_size as u32 });
        }

        // event_id: Guid
        let event_id = Guid::read(r)?;

        // tentative: Bool
        let tentative = crate::util::read_u8_le(r)? != 0;
        Ok(Self {
            event_id,
            tentative,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_CALENDAR_EVENT_SIGNUP {}

