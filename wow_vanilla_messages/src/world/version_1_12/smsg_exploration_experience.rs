use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::Area;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/exp/smsg_exploration_experience.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/exp/smsg_exploration_experience.wowm#L3):
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

impl ServerMessage for SMSG_EXPLORATION_EXPERIENCE {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // area: Area
        w.write_all(&(self.area.as_int() as u32).to_le_bytes())?;

        // experience: u32
        w.write_all(&self.experience.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x01f8;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        8
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
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

