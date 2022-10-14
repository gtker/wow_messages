use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/cmsg_world_state_ui_timer_update.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/cmsg_world_state_ui_timer_update.wowm#L3):
/// ```text
/// cmsg CMSG_WORLD_STATE_UI_TIMER_UPDATE = 0x04F6 {
/// }
/// ```
pub struct CMSG_WORLD_STATE_UI_TIMER_UPDATE {
}

impl crate::Message for CMSG_WORLD_STATE_UI_TIMER_UPDATE {
    const OPCODE: u32 = 0x04f6;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_WORLD_STATE_UI_TIMER_UPDATE {}

