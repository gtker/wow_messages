use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/smsg_set_phase_shift.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/smsg_set_phase_shift.wowm#L1):
/// ```text
/// smsg SMSG_SET_PHASE_SHIFT = 0x047C {
///     u32 new_phase;
/// }
/// ```
pub struct SMSG_SET_PHASE_SHIFT {
    pub new_phase: u32,
}

impl crate::private::Sealed for SMSG_SET_PHASE_SHIFT {}
impl crate::Message for SMSG_SET_PHASE_SHIFT {
    const OPCODE: u32 = 0x047c;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // new_phase: u32
        w.write_all(&self.new_phase.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x047C, size: body_size });
        }

        // new_phase: u32
        let new_phase = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            new_phase,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SET_PHASE_SHIFT {}

