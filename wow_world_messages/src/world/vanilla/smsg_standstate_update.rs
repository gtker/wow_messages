use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::UnitStandState;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_standstate_update.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_standstate_update.wowm#L3):
/// ```text
/// smsg SMSG_STANDSTATE_UPDATE = 0x029D {
///     UnitStandState state;
/// }
/// ```
pub struct SMSG_STANDSTATE_UPDATE {
    pub state: UnitStandState,
}

impl crate::Message for SMSG_STANDSTATE_UPDATE {
    const OPCODE: u32 = 0x029d;

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // state: UnitStandState
        w.write_all(&(self.state.as_int() as u8).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // state: UnitStandState
        let state: UnitStandState = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            state,
        })
    }

}
impl ServerMessage for SMSG_STANDSTATE_UPDATE {}

