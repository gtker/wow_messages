use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::TimerType;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_stop_mirror_timer.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_stop_mirror_timer.wowm#L3):
/// ```text
/// smsg SMSG_STOP_MIRROR_TIMER = 0x01DB {
///     TimerType timer;
/// }
/// ```
pub struct SMSG_STOP_MIRROR_TIMER {
    pub timer: TimerType,
}

impl ServerMessage for SMSG_STOP_MIRROR_TIMER {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // timer: TimerType
        w.write_all(&(self.timer.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x01db;

    fn server_size(&self) -> u16 {
        8
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

