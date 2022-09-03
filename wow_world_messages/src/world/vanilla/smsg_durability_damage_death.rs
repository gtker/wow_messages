use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::ServerMessage;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_durability_damage_death.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_durability_damage_death.wowm#L3):
/// ```text
/// smsg SMSG_DURABILITY_DAMAGE_DEATH = 0x02BD {
/// }
/// ```
pub struct SMSG_DURABILITY_DAMAGE_DEATH {
}

impl crate::Message for SMSG_DURABILITY_DAMAGE_DEATH {
    const OPCODE: u32 = 0x02bd;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        Ok(Self {
        })
    }

}
impl ServerMessage for SMSG_DURABILITY_DAMAGE_DEATH {}

