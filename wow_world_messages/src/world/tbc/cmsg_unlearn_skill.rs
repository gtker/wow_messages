use std::io::{Read, Write};

use crate::tbc::Skill;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/cmsg_unlearn_skill.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/cmsg_unlearn_skill.wowm#L1):
/// ```text
/// cmsg CMSG_UNLEARN_SKILL = 0x0202 {
///     (u32)Skill skill;
/// }
/// ```
pub struct CMSG_UNLEARN_SKILL {
    pub skill: Skill,
}

impl crate::private::Sealed for CMSG_UNLEARN_SKILL {}
impl crate::Message for CMSG_UNLEARN_SKILL {
    const OPCODE: u32 = 0x0202;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // skill: Skill
        w.write_all(&u32::from(self.skill.as_int()).to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0202, size: body_size as u32 });
        }

        // skill: Skill
        let skill: Skill = (crate::util::read_u32_le(&mut r)? as u16).try_into()?;

        Ok(Self {
            skill,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_UNLEARN_SKILL {}

