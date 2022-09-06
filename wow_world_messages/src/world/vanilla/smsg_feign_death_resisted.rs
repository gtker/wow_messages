use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_feign_death_resisted.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_feign_death_resisted.wowm#L3):
/// ```text
/// smsg SMSG_FEIGN_DEATH_RESISTED = 0x02B4 {
/// }
/// ```
pub struct SMSG_FEIGN_DEATH_RESISTED {
}

impl crate::Message for SMSG_FEIGN_DEATH_RESISTED {
    const OPCODE: u32 = 0x02b4;

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
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_FEIGN_DEATH_RESISTED {}

