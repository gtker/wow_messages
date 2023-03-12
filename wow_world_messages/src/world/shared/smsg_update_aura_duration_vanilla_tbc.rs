use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_update_aura_duration.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_update_aura_duration.wowm#L3):
/// ```text
/// smsg SMSG_UPDATE_AURA_DURATION = 0x0137 {
///     u8 aura_slot;
///     u32 aura_duration;
/// }
/// ```
pub struct SMSG_UPDATE_AURA_DURATION {
    pub aura_slot: u8,
    pub aura_duration: u32,
}

impl crate::Message for SMSG_UPDATE_AURA_DURATION {
    const OPCODE: u32 = 0x0137;

    fn size_without_header(&self) -> u32 {
        5
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // aura_slot: u8
        w.write_all(&self.aura_slot.to_le_bytes())?;

        // aura_duration: u32
        w.write_all(&self.aura_duration.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 5 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0137, size: body_size as u32 });
        }

        // aura_slot: u8
        let aura_slot = crate::util::read_u8_le(&mut r)?;

        // aura_duration: u32
        let aura_duration = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            aura_slot,
            aura_duration,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_UPDATE_AURA_DURATION {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_UPDATE_AURA_DURATION {}

