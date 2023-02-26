use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/cmsg_remove_glyph.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/cmsg_remove_glyph.wowm#L1):
/// ```text
/// cmsg CMSG_REMOVE_GLYPH = 0x048A {
///     u32 glyph;
/// }
/// ```
pub struct CMSG_REMOVE_GLYPH {
    pub glyph: u32,
}

impl crate::Message for CMSG_REMOVE_GLYPH {
    const OPCODE: u32 = 0x048a;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // glyph: u32
        w.write_all(&self.glyph.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x048A, size: body_size as u32 });
        }

        // glyph: u32
        let glyph = crate::util::read_u32_le(r)?;

        Ok(Self {
            glyph,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_REMOVE_GLYPH {}

