use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/duel/smsg_duel_countdown.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/duel/smsg_duel_countdown.wowm#L3):
/// ```text
/// smsg SMSG_DUEL_COUNTDOWN = 0x02B7 {
///     u32 time_in_seconds;
/// }
/// ```
pub struct SMSG_DUEL_COUNTDOWN {
    pub time_in_seconds: u32,
}

impl crate::Message for SMSG_DUEL_COUNTDOWN {
    const OPCODE: u32 = 0x02b7;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // time_in_seconds: u32
        w.write_all(&self.time_in_seconds.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // time_in_seconds: u32
        let time_in_seconds = crate::util::read_u32_le(r)?;

        Ok(Self {
            time_in_seconds,
        })
    }

}
#[cfg(any(feature = "vanilla", feature = "tbc"))]
impl crate::helper::shared::vanilla_tbc::ServerMessage for SMSG_DUEL_COUNTDOWN {}

