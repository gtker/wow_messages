use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::TimerType;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
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

impl ServerMessage for SMSG_PAUSE_MIRROR_TIMER {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // timer: TimerType
        w.write_all(&(self.timer.as_int() as u32).to_le_bytes())?;

        // is_frozen: u8
        w.write_all(&self.is_frozen.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x01da;

    fn server_size(&self) -> u16 {
        9
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

