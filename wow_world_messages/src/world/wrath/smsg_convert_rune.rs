use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_convert_rune.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_convert_rune.wowm#L1):
/// ```text
/// smsg SMSG_CONVERT_RUNE = 0x0486 {
///     u8 index;
///     u8 new_type;
/// }
/// ```
pub struct SMSG_CONVERT_RUNE {
    pub index: u8,
    pub new_type: u8,
}

impl crate::Message for SMSG_CONVERT_RUNE {
    const OPCODE: u32 = 0x0486;

    fn size_without_header(&self) -> u32 {
        2
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // index: u8
        w.write_all(&self.index.to_le_bytes())?;

        // new_type: u8
        w.write_all(&self.new_type.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 2 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0486, size: body_size as u32 });
        }

        // index: u8
        let index = crate::util::read_u8_le(r)?;

        // new_type: u8
        let new_type = crate::util::read_u8_le(r)?;

        Ok(Self {
            index,
            new_type,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CONVERT_RUNE {}

