use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/duel/smsg_duel_complete.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/duel/smsg_duel_complete.wowm#L3):
/// ```text
/// smsg SMSG_DUEL_COMPLETE = 0x016A {
///     u8 ended_without_interruption;
/// }
/// ```
pub struct SMSG_DUEL_COMPLETE {
    pub ended_without_interruption: u8,
}

impl ServerMessage for SMSG_DUEL_COMPLETE {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // ended_without_interruption: u8
        w.write_all(&self.ended_without_interruption.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x016a;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        1
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // ended_without_interruption: u8
        let ended_without_interruption = crate::util::read_u8_le(r)?;

        Ok(Self {
            ended_without_interruption,
        })
    }

}

