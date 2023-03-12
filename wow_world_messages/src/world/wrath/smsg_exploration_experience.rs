use std::io::{Read, Write};

use crate::wrath::Area;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/exp/smsg_exploration_experience.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/exp/smsg_exploration_experience.wowm#L1):
/// ```text
/// smsg SMSG_EXPLORATION_EXPERIENCE = 0x01F8 {
///     Area area;
///     u32 experience;
/// }
/// ```
pub struct SMSG_EXPLORATION_EXPERIENCE {
    pub area: Area,
    pub experience: u32,
}

impl crate::Message for SMSG_EXPLORATION_EXPERIENCE {
    const OPCODE: u32 = 0x01f8;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // area: Area
        w.write_all(&u32::from(self.area.as_int()).to_le_bytes())?;

        // experience: u32
        w.write_all(&self.experience.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01F8, size: body_size as u32 });
        }

        // area: Area
        let area: Area = crate::util::read_u32_le(&mut r)?.try_into()?;

        // experience: u32
        let experience = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            area,
            experience,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_EXPLORATION_EXPERIENCE {}

