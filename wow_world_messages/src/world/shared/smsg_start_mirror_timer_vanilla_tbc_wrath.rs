use std::io::{Read, Write};

use wow_world_base::shared::timer_type_vanilla_vanilla_tbc_wrath::TimerType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_start_mirror_timer.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_start_mirror_timer.wowm#L3):
/// ```text
/// smsg SMSG_START_MIRROR_TIMER = 0x01D9 {
///     TimerType timer;
///     u32 time_remaining;
///     u32 duration;
///     u32 scale;
///     Bool is_frozen;
///     u32 id;
/// }
/// ```
pub struct SMSG_START_MIRROR_TIMER {
    pub timer: TimerType,
    pub time_remaining: u32,
    pub duration: u32,
    pub scale: u32,
    pub is_frozen: bool,
    pub id: u32,
}

impl crate::private::Sealed for SMSG_START_MIRROR_TIMER {}
impl crate::Message for SMSG_START_MIRROR_TIMER {
    const OPCODE: u32 = 0x01d9;

    fn size_without_header(&self) -> u32 {
        21
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // timer: TimerType
        w.write_all(&(self.timer.as_int().to_le_bytes()))?;

        // time_remaining: u32
        w.write_all(&self.time_remaining.to_le_bytes())?;

        // duration: u32
        w.write_all(&self.duration.to_le_bytes())?;

        // scale: u32
        w.write_all(&self.scale.to_le_bytes())?;

        // is_frozen: Bool
        w.write_all(u8::from(self.is_frozen).to_le_bytes().as_slice())?;

        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 21 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01D9, size: body_size });
        }

        // timer: TimerType
        let timer: TimerType = crate::util::read_u32_le(&mut r)?.try_into()?;

        // time_remaining: u32
        let time_remaining = crate::util::read_u32_le(&mut r)?;

        // duration: u32
        let duration = crate::util::read_u32_le(&mut r)?;

        // scale: u32
        let scale = crate::util::read_u32_le(&mut r)?;

        // is_frozen: Bool
        let is_frozen = crate::util::read_u8_le(&mut r)? != 0;

        // id: u32
        let id = crate::util::read_u32_le(&mut r)?;

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
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_START_MIRROR_TIMER {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_START_MIRROR_TIMER {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_START_MIRROR_TIMER {}

