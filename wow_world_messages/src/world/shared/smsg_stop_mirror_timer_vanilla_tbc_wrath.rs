use std::io::{Read, Write};

use wow_world_base::shared::timer_type_vanilla_vanilla_tbc_wrath::TimerType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_stop_mirror_timer.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_stop_mirror_timer.wowm#L3):
/// ```text
/// smsg SMSG_STOP_MIRROR_TIMER = 0x01DB {
///     TimerType timer;
/// }
/// ```
pub struct SMSG_STOP_MIRROR_TIMER {
    pub timer: TimerType,
}

impl crate::private::Sealed for SMSG_STOP_MIRROR_TIMER {}
impl crate::Message for SMSG_STOP_MIRROR_TIMER {
    const OPCODE: u32 = 0x01db;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // timer: TimerType
        w.write_all(&(self.timer.as_int().to_le_bytes()))?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01DB, size: body_size });
        }

        // timer: TimerType
        let timer: TimerType = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            timer,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_STOP_MIRROR_TIMER {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_STOP_MIRROR_TIMER {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_STOP_MIRROR_TIMER {}

