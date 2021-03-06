use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::PetitionTurnInResult;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/smsg_turn_in_petition_results.wowm:12`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_turn_in_petition_results.wowm#L12):
/// ```text
/// smsg SMSG_TURN_IN_PETITION_RESULTS = 0x01C5 {
///     PetitionTurnInResult result;
/// }
/// ```
pub struct SMSG_TURN_IN_PETITION_RESULTS {
    pub result: PetitionTurnInResult,
}

impl ServerMessage for SMSG_TURN_IN_PETITION_RESULTS {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // result: PetitionTurnInResult
        w.write_all(&(self.result.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x01c5;

    fn server_size(&self) -> u16 {
        8
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // result: PetitionTurnInResult
        let result: PetitionTurnInResult = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            result,
        })
    }

}

