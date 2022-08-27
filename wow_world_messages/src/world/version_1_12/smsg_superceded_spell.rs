use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_superceded_spell.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_superceded_spell.wowm#L3):
/// ```text
/// smsg SMSG_SUPERCEDED_SPELL = 0x012C {
///     u16 new_spell_id;
///     u16 old_spell_id;
/// }
/// ```
pub struct SMSG_SUPERCEDED_SPELL {
    pub new_spell_id: u16,
    pub old_spell_id: u16,
}

impl ServerMessage for SMSG_SUPERCEDED_SPELL {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // new_spell_id: u16
        w.write_all(&self.new_spell_id.to_le_bytes())?;

        // old_spell_id: u16
        w.write_all(&self.old_spell_id.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x012c;

    fn server_size(&self) -> u16 {
        8
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // new_spell_id: u16
        let new_spell_id = crate::util::read_u16_le(r)?;

        // old_spell_id: u16
        let old_spell_id = crate::util::read_u16_le(r)?;

        Ok(Self {
            new_spell_id,
            old_spell_id,
        })
    }

}

