use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::TimerType;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_stop_mirror_timer.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_stop_mirror_timer.wowm#L3):
/// ```text
/// smsg SMSG_STOP_MIRROR_TIMER = 0x01DB {
///     TimerType timer;
/// }
/// ```
pub struct SMSG_STOP_MIRROR_TIMER {
    pub timer: TimerType,
}

impl crate::Message for SMSG_STOP_MIRROR_TIMER {
    const OPCODE: u32 = 0x01db;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // timer: TimerType
        w.write_all(&(self.timer.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // timer: TimerType
        let timer: TimerType = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            timer,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_STOP_MIRROR_TIMER {}

