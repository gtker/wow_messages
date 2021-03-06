use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_removed_spell.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_removed_spell.wowm#L3):
/// ```text
/// smsg SMSG_REMOVED_SPELL = 0x0203 {
///     u16 spell_id;
/// }
/// ```
pub struct SMSG_REMOVED_SPELL {
    pub spell_id: u16,
}

impl ServerMessage for SMSG_REMOVED_SPELL {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // spell_id: u16
        w.write_all(&self.spell_id.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0203;

    fn server_size(&self) -> u16 {
        6
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 2 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // spell_id: u16
        let spell_id = crate::util::read_u16_le(r)?;

        Ok(Self {
            spell_id,
        })
    }

}

