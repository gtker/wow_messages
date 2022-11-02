use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::Area;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
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

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // area: Area
        w.write_all(&(self.area.as_int() as u32).to_le_bytes())?;

        // experience: u32
        w.write_all(&self.experience.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01F8, size: body_size as u32 });
        }

        // area: Area
        let area: Area = crate::util::read_u32_le(r)?.try_into()?;

        // experience: u32
        let experience = crate::util::read_u32_le(r)?;

        Ok(Self {
            area,
            experience,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_EXPLORATION_EXPERIENCE {}

