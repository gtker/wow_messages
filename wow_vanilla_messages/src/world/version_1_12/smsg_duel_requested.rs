use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/duel/smsg_duel_requested.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/duel/smsg_duel_requested.wowm#L3):
/// ```text
/// smsg SMSG_DUEL_REQUESTED = 0x0167 {
///     Guid initiator_guid;
///     Guid target_guid;
/// }
/// ```
pub struct SMSG_DUEL_REQUESTED {
    pub initiator_guid: Guid,
    pub target_guid: Guid,
}

impl ServerMessage for SMSG_DUEL_REQUESTED {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // initiator_guid: Guid
        w.write_all(&self.initiator_guid.guid().to_le_bytes())?;

        // target_guid: Guid
        w.write_all(&self.target_guid.guid().to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0167;

    fn server_size(&self) -> u16 {
        20
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // initiator_guid: Guid
        let initiator_guid = Guid::read(r)?;

        // target_guid: Guid
        let target_guid = Guid::read(r)?;

        Ok(Self {
            initiator_guid,
            target_guid,
        })
    }

}

