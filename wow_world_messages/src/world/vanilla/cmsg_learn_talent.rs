use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Copy, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/cmsg_learn_talent.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/cmsg_learn_talent.wowm#L3):
/// ```text
/// cmsg CMSG_LEARN_TALENT = 0x0251 {
///     u32 talent_id;
///     u32 requested_rank;
/// }
/// ```
pub struct CMSG_LEARN_TALENT {
    pub talent_id: u32,
    pub requested_rank: u32,
}

impl crate::Message for CMSG_LEARN_TALENT {
    const OPCODE: u32 = 0x0251;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // talent_id: u32
        w.write_all(&self.talent_id.to_le_bytes())?;

        // requested_rank: u32
        w.write_all(&self.requested_rank.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // talent_id: u32
        let talent_id = crate::util::read_u32_le(r)?;

        // requested_rank: u32
        let requested_rank = crate::util::read_u32_le(r)?;

        Ok(Self {
            talent_id,
            requested_rank,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_LEARN_TALENT {}

