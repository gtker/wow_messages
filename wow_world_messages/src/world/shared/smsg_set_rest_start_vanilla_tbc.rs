use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/exp/smsg_set_rest_start.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/exp/smsg_set_rest_start.wowm#L3):
/// ```text
/// smsg SMSG_SET_REST_START = 0x021E {
///     u32 unknown1;
/// }
/// ```
pub struct SMSG_SET_REST_START {
    /// cmangos/mangoszero: unknown, may be rest state time or experience
    ///
    pub unknown1: u32,
}

impl crate::private::Sealed for SMSG_SET_REST_START {}
impl crate::Message for SMSG_SET_REST_START {
    const OPCODE: u32 = 0x021e;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x021E, size: body_size });
        }

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            unknown1,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_SET_REST_START {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_SET_REST_START {}

