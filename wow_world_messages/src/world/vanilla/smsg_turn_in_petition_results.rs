use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::PetitionTurnInResult;
use crate::world::vanilla::ServerMessage;
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

impl crate::Message for SMSG_TURN_IN_PETITION_RESULTS {
    const OPCODE: u32 = 0x01c5;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // result: PetitionTurnInResult
        w.write_all(&(self.result.as_int() as u32).to_le_bytes())?;

        Ok(())
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
impl ServerMessage for SMSG_TURN_IN_PETITION_RESULTS {}

