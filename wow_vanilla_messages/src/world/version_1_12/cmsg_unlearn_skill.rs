use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/cmsg_unlearn_skill.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/cmsg_unlearn_skill.wowm#L3):
/// ```text
/// cmsg CMSG_UNLEARN_SKILL = 0x0202 {
///     u32 skill_id;
/// }
/// ```
pub struct CMSG_UNLEARN_SKILL {
    pub skill_id: u32,
}

impl ClientMessage for CMSG_UNLEARN_SKILL {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // skill_id: u32
        w.write_all(&self.skill_id.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0202;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        4
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // skill_id: u32
        let skill_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            skill_id,
        })
    }

}

