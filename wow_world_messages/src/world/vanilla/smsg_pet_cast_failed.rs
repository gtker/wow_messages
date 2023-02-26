use crate::vanilla::SpellCastResult;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/smsg_pet_cast_failed.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/smsg_pet_cast_failed.wowm#L1):
/// ```text
/// smsg SMSG_PET_CAST_FAILED = 0x0138 {
///     u32 id;
///     u8 unknown1;
///     SpellCastResult result;
/// }
/// ```
pub struct SMSG_PET_CAST_FAILED {
    pub id: u32,
    /// vmangos sets to 2 and cmangos sets to 0.
    ///
    pub unknown1: u8,
    pub result: SpellCastResult,
}

impl crate::Message for SMSG_PET_CAST_FAILED {
    const OPCODE: u32 = 0x0138;

    fn size_without_header(&self) -> u32 {
        6
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        // result: SpellCastResult
        w.write_all(&(self.result.as_int() as u8).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 6 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0138, size: body_size as u32 });
        }

        // id: u32
        let id = crate::util::read_u32_le(r)?;

        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(r)?;

        // result: SpellCastResult
        let result: SpellCastResult = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            id,
            unknown1,
            result,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_PET_CAST_FAILED {}

