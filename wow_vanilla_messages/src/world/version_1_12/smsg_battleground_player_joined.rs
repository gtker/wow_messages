use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/smsg_battleground_player_joined.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/smsg_battleground_player_joined.wowm#L3):
/// ```text
/// smsg SMSG_BATTLEGROUND_PLAYER_JOINED = 0x02EC {
///     Guid player_guid;
/// }
/// ```
pub struct SMSG_BATTLEGROUND_PLAYER_JOINED {
    pub player_guid: Guid,
}

impl ServerMessage for SMSG_BATTLEGROUND_PLAYER_JOINED {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // player_guid: Guid
        w.write_all(&self.player_guid.guid().to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x02ec;

    fn server_size(&self) -> u16 {
        12
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // player_guid: Guid
        let player_guid = Guid::read(r)?;

        Ok(Self {
            player_guid,
        })
    }

}

