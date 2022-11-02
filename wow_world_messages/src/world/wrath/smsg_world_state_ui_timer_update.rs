use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_world_state_ui_timer_update.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_world_state_ui_timer_update.wowm#L3):
/// ```text
/// smsg SMSG_WORLD_STATE_UI_TIMER_UPDATE = 0x04F7 {
///     u32 time;
/// }
/// ```
pub struct SMSG_WORLD_STATE_UI_TIMER_UPDATE {
    /// Seconds since Unix Epoch
    ///
    pub time: u32,
}

impl crate::Message for SMSG_WORLD_STATE_UI_TIMER_UPDATE {
    const OPCODE: u32 = 0x04f7;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // time: u32
        w.write_all(&self.time.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04F7, size: body_size as u32 });
        }

        // time: u32
        let time = crate::util::read_u32_le(r)?;

        Ok(Self {
            time,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_WORLD_STATE_UI_TIMER_UPDATE {}

