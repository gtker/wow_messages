use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/resurrect/smsg_corpse_reclaim_delay.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/resurrect/smsg_corpse_reclaim_delay.wowm#L3):
/// ```text
/// smsg SMSG_CORPSE_RECLAIM_DELAY = 0x0269 {
///     u32 delay_in_seconds;
/// }
/// ```
pub struct SMSG_CORPSE_RECLAIM_DELAY {
    pub delay_in_seconds: u32,
}

impl crate::Message for SMSG_CORPSE_RECLAIM_DELAY {
    const OPCODE: u32 = 0x0269;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // delay_in_seconds: u32
        w.write_all(&self.delay_in_seconds.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0269, size: body_size as u32 });
        }

        // delay_in_seconds: u32
        let delay_in_seconds = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            delay_in_seconds,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_CORPSE_RECLAIM_DELAY {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_CORPSE_RECLAIM_DELAY {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CORPSE_RECLAIM_DELAY {}

