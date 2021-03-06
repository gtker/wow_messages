use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_set_pct_spell_modifier.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_set_pct_spell_modifier.wowm#L3):
/// ```text
/// smsg SMSG_SET_PCT_SPELL_MODIFIER = 0x0267 {
///     u8 eff;
///     u8 op;
///     u32 value;
/// }
/// ```
pub struct SMSG_SET_PCT_SPELL_MODIFIER {
    pub eff: u8,
    pub op: u8,
    pub value: u32,
}

impl ServerMessage for SMSG_SET_PCT_SPELL_MODIFIER {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // eff: u8
        w.write_all(&self.eff.to_le_bytes())?;

        // op: u8
        w.write_all(&self.op.to_le_bytes())?;

        // value: u32
        w.write_all(&self.value.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0267;

    fn server_size(&self) -> u16 {
        10
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 6 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // eff: u8
        let eff = crate::util::read_u8_le(r)?;

        // op: u8
        let op = crate::util::read_u8_le(r)?;

        // value: u32
        let value = crate::util::read_u32_le(r)?;

        Ok(Self {
            eff,
            op,
            value,
        })
    }

}

