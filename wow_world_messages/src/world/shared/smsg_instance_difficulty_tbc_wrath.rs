use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/smsg_instance_difficulty.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/smsg_instance_difficulty.wowm#L1):
/// ```text
/// smsg SMSG_INSTANCE_DIFFICULTY = 0x033B {
///     u32 difficulty;
///     Bool32 dynamic_difficulty;
/// }
/// ```
pub struct SMSG_INSTANCE_DIFFICULTY {
    pub difficulty: u32,
    pub dynamic_difficulty: bool,
}

impl crate::Message for SMSG_INSTANCE_DIFFICULTY {
    const OPCODE: u32 = 0x033b;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // difficulty: u32
        w.write_all(&self.difficulty.to_le_bytes())?;

        // dynamic_difficulty: Bool32
        w.write_all(u32::from(self.dynamic_difficulty).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x033B, size: body_size as u32 });
        }

        // difficulty: u32
        let difficulty = crate::util::read_u32_le(r)?;

        // dynamic_difficulty: Bool32
        let dynamic_difficulty = crate::util::read_u32_le(r)? != 0;
        Ok(Self {
            difficulty,
            dynamic_difficulty,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_INSTANCE_DIFFICULTY {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_INSTANCE_DIFFICULTY {}

