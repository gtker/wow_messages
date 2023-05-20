use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/cmsg_alter_appearance.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/cmsg_alter_appearance.wowm#L1):
/// ```text
/// cmsg CMSG_ALTER_APPEARANCE = 0x0426 {
///     u32 hair;
///     u32 hair_color;
///     u32 facial_hair;
/// }
/// ```
pub struct CMSG_ALTER_APPEARANCE {
    pub hair: u32,
    pub hair_color: u32,
    pub facial_hair: u32,
}

impl crate::private::Sealed for CMSG_ALTER_APPEARANCE {}
impl crate::Message for CMSG_ALTER_APPEARANCE {
    const OPCODE: u32 = 0x0426;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // hair: u32
        w.write_all(&self.hair.to_le_bytes())?;

        // hair_color: u32
        w.write_all(&self.hair_color.to_le_bytes())?;

        // facial_hair: u32
        w.write_all(&self.facial_hair.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0426, size: body_size });
        }

        // hair: u32
        let hair = crate::util::read_u32_le(&mut r)?;

        // hair_color: u32
        let hair_color = crate::util::read_u32_le(&mut r)?;

        // facial_hair: u32
        let facial_hair = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            hair,
            hair_color,
            facial_hair,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_ALTER_APPEARANCE {}

