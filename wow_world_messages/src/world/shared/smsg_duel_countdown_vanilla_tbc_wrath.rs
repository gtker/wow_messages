use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
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

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // time_in_seconds: u32
        w.write_all(&self.time_in_seconds.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02B7, size: body_size as u32 });
        }

        // time_in_seconds: u32
        let time_in_seconds = crate::util::read_u32_le(r)?;

        Ok(Self {
            time_in_seconds,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_DUEL_COUNTDOWN {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_DUEL_COUNTDOWN {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_DUEL_COUNTDOWN {}

