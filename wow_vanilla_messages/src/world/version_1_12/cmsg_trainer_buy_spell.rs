use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
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

impl ClientMessage for CMSG_TRAINER_BUY_SPELL {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x01b2;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        12
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        // id: u32
        let id = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            id,
        })
    }

}

