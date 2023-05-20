use std::io::{Read, Write};

use crate::vanilla::Talent;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/cmsg_learn_talent.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/cmsg_learn_talent.wowm#L1):
/// ```text
/// cmsg CMSG_LEARN_TALENT = 0x0251 {
///     Talent talent;
///     u32 requested_rank;
/// }
/// ```
pub struct CMSG_LEARN_TALENT {
    pub talent: Talent,
    pub requested_rank: u32,
}

impl crate::private::Sealed for CMSG_LEARN_TALENT {}
impl crate::Message for CMSG_LEARN_TALENT {
    const OPCODE: u32 = 0x0251;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // talent: Talent
        w.write_all(&(self.talent.as_int().to_le_bytes()))?;

        // requested_rank: u32
        w.write_all(&self.requested_rank.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0251, size: body_size });
        }

        // talent: Talent
        let talent: Talent = crate::util::read_u32_le(&mut r)?.try_into()?;

        // requested_rank: u32
        let requested_rank = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            talent,
            requested_rank,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_LEARN_TALENT {}

