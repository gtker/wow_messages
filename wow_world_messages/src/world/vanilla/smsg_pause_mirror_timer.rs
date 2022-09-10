use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::TimerType;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// According to cmangos: 'Default UI handler for this is bugged, args dont match. Gotta do a full update with `SMSG_START_MIRROR_TIMER` to avoid lua errors.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_pause_mirror_timer.wowm:5`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_pause_mirror_timer.wowm#L5):
/// ```text
/// smsg SMSG_PAUSE_MIRROR_TIMER = 0x01DA {
///     TimerType timer;
///     u8 is_frozen;
/// }
/// ```
pub struct SMSG_PAUSE_MIRROR_TIMER {
    pub timer: TimerType,
    pub is_frozen: u8,
}

impl crate::Message for SMSG_PAUSE_MIRROR_TIMER {
    const OPCODE: u32 = 0x01da;

    fn size_without_header(&self) -> u32 {
        5
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // timer: TimerType
        w.write_all(&(self.timer.as_int() as u32).to_le_bytes())?;

        // is_frozen: u8
        w.write_all(&self.is_frozen.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 5 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // timer: TimerType
        let timer: TimerType = crate::util::read_u32_le(r)?.try_into()?;

        // is_frozen: u8
        let is_frozen = crate::util::read_u8_le(r)?;

        Ok(Self {
            timer,
            is_frozen,
        })
    }

}
#[cfg(any(feature = "vanilla", feature = "tbc"))]
impl crate::helper::shared::vanilla_tbc::ServerMessage for SMSG_PAUSE_MIRROR_TIMER {}

