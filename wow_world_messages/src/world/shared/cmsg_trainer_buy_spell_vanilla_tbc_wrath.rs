use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/cmsg_trainer_buy_spell.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/cmsg_trainer_buy_spell.wowm#L3):
/// ```text
/// cmsg CMSG_TRAINER_BUY_SPELL = 0x01B2 {
///     Guid guid;
///     u32 id;
/// }
/// ```
pub struct CMSG_TRAINER_BUY_SPELL {
    pub guid: Guid,
    pub id: u32,
}

impl crate::private::Sealed for CMSG_TRAINER_BUY_SPELL {}
impl crate::Message for CMSG_TRAINER_BUY_SPELL {
    const OPCODE: u32 = 0x01b2;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01B2, size: body_size });
        }

        // guid: Guid
        let guid = Guid::read(&mut r)?;

        // id: u32
        let id = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            guid,
            id,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_TRAINER_BUY_SPELL {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_TRAINER_BUY_SPELL {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_TRAINER_BUY_SPELL {}

