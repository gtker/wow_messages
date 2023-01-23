use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_calendar_clear_pending_action.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_calendar_clear_pending_action.wowm#L1):
/// ```text
/// smsg SMSG_CALENDAR_CLEAR_PENDING_ACTION = 0x04BB {
/// }
/// ```
pub struct SMSG_CALENDAR_CLEAR_PENDING_ACTION {
}

impl crate::Message for SMSG_CALENDAR_CLEAR_PENDING_ACTION {
    const OPCODE: u32 = 0x04bb;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04BB, size: body_size as u32 });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_CALENDAR_CLEAR_PENDING_ACTION {}

