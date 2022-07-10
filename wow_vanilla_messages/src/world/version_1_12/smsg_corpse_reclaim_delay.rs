use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/resurrect/smsg_corpse_reclaim_delay.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/resurrect/smsg_corpse_reclaim_delay.wowm#L3):
/// ```text
/// smsg SMSG_CORPSE_RECLAIM_DELAY = 0x0269 {
///     u32 delay_in_seconds;
/// }
/// ```
pub struct SMSG_CORPSE_RECLAIM_DELAY {
    pub delay_in_seconds: u32,
}

impl ServerMessage for SMSG_CORPSE_RECLAIM_DELAY {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // delay_in_seconds: u32
        w.write_all(&self.delay_in_seconds.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0269;

    fn server_size(&self) -> u16 {
        8
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // delay_in_seconds: u32
        let delay_in_seconds = crate::util::read_u32_le(r)?;

        Ok(Self {
            delay_in_seconds,
        })
    }

}

