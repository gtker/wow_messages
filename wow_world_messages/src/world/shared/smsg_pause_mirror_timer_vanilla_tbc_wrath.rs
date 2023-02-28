use wow_world_base::shared::timer_type_vanilla_vanilla_tbc_wrath::TimerType;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// According to cmangos: 'Default UI handler for this is bugged, args dont match. Gotta do a full update with `SMSG_START_MIRROR_TIMER` to avoid lua errors.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_pause_mirror_timer.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_pause_mirror_timer.wowm#L3):
/// ```text
/// smsg SMSG_PAUSE_MIRROR_TIMER = 0x01DA {
///     TimerType timer;
///     Bool is_frozen;
/// }
/// ```
pub struct SMSG_PAUSE_MIRROR_TIMER {
    pub timer: TimerType,
    pub is_frozen: bool,
}

impl crate::Message for SMSG_PAUSE_MIRROR_TIMER {
    const OPCODE: u32 = 0x01da;

    fn size_without_header(&self) -> u32 {
        5
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // timer: TimerType
        w.write_all(&u32::from(self.timer.as_int()).to_le_bytes())?;

        // is_frozen: Bool
        w.write_all(u8::from(self.is_frozen).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 5 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01DA, size: body_size as u32 });
        }

        // timer: TimerType
        let timer: TimerType = crate::util::read_u32_le(&mut r)?.try_into()?;

        // is_frozen: Bool
        let is_frozen = crate::util::read_u8_le(&mut r)? != 0;

        Ok(Self {
            timer,
            is_frozen,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_PAUSE_MIRROR_TIMER {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_PAUSE_MIRROR_TIMER {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_PAUSE_MIRROR_TIMER {}

