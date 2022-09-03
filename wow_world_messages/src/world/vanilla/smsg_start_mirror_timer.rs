use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::TimerType;
use crate::world::vanilla::ServerMessage;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_start_mirror_timer.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_start_mirror_timer.wowm#L3):
/// ```text
/// smsg SMSG_START_MIRROR_TIMER = 0x01D9 {
///     TimerType timer;
///     u32 time_remaining;
///     u32 duration;
///     u32 scale;
///     u8 is_frozen;
///     u32 id;
/// }
/// ```
pub struct SMSG_START_MIRROR_TIMER {
    pub timer: TimerType,
    pub time_remaining: u32,
    pub duration: u32,
    pub scale: u32,
    pub is_frozen: u8,
    pub id: u32,
}

impl crate::Message for SMSG_START_MIRROR_TIMER {
    const OPCODE: u32 = 0x01d9;

    fn size_without_header(&self) -> u32 {
        21
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // timer: TimerType
        w.write_all(&(self.timer.as_int() as u32).to_le_bytes())?;

        // time_remaining: u32
        w.write_all(&self.time_remaining.to_le_bytes())?;

        // duration: u32
        w.write_all(&self.duration.to_le_bytes())?;

        // scale: u32
        w.write_all(&self.scale.to_le_bytes())?;

        // is_frozen: u8
        w.write_all(&self.is_frozen.to_le_bytes())?;

        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 21 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // timer: TimerType
        let timer: TimerType = crate::util::read_u32_le(r)?.try_into()?;

        // time_remaining: u32
        let time_remaining = crate::util::read_u32_le(r)?;

        // duration: u32
        let duration = crate::util::read_u32_le(r)?;

        // scale: u32
        let scale = crate::util::read_u32_le(r)?;

        // is_frozen: u8
        let is_frozen = crate::util::read_u8_le(r)?;

        // id: u32
        let id = crate::util::read_u32_le(r)?;

        Ok(Self {
            timer,
            time_remaining,
            duration,
            scale,
            is_frozen,
            id,
        })
    }

}
impl ServerMessage for SMSG_START_MIRROR_TIMER {}

