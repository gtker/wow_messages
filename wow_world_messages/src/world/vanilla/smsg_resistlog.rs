use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Structure as comment on `https://github1s.com/mangoszero/server/blob/HEAD/src/game/Server/Opcodes.h#L525`.
/// Not used in azerothcore/trinitycore/mangostwo/arcemu.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_resistlog.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_resistlog.wowm#L3):
/// ```text
/// smsg SMSG_RESISTLOG = 0x01D6 {
///     Guid guid1;
///     Guid guid2;
///     u32 unknown1;
///     f32 unknown2;
///     f32 unknown3;
///     u32 unknown4;
///     u32 unknown5;
/// }
/// ```
pub struct SMSG_RESISTLOG {
    pub guid1: Guid,
    pub guid2: Guid,
    pub unknown1: u32,
    pub unknown2: f32,
    pub unknown3: f32,
    pub unknown4: u32,
    pub unknown5: u32,
}

impl crate::private::Sealed for SMSG_RESISTLOG {}
impl crate::Message for SMSG_RESISTLOG {
    const OPCODE: u32 = 0x01d6;

    fn size_without_header(&self) -> u32 {
        36
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guid1: Guid
        w.write_all(&self.guid1.guid().to_le_bytes())?;

        // guid2: Guid
        w.write_all(&self.guid2.guid().to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: f32
        w.write_all(&self.unknown2.to_le_bytes())?;

        // unknown3: f32
        w.write_all(&self.unknown3.to_le_bytes())?;

        // unknown4: u32
        w.write_all(&self.unknown4.to_le_bytes())?;

        // unknown5: u32
        w.write_all(&self.unknown5.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 36 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01D6, size: body_size });
        }

        // guid1: Guid
        let guid1 = Guid::read(&mut r)?;

        // guid2: Guid
        let guid2 = Guid::read(&mut r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(&mut r)?;

        // unknown2: f32
        let unknown2 = crate::util::read_f32_le(&mut r)?;

        // unknown3: f32
        let unknown3 = crate::util::read_f32_le(&mut r)?;

        // unknown4: u32
        let unknown4 = crate::util::read_u32_le(&mut r)?;

        // unknown5: u32
        let unknown5 = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            guid1,
            guid2,
            unknown1,
            unknown2,
            unknown3,
            unknown4,
            unknown5,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_RESISTLOG {}

