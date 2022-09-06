use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/smsg_battleground_player_joined.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/smsg_battleground_player_joined.wowm#L3):
/// ```text
/// smsg SMSG_BATTLEGROUND_PLAYER_JOINED = 0x02EC {
///     Guid player_guid;
/// }
/// ```
pub struct SMSG_BATTLEGROUND_PLAYER_JOINED {
    pub player_guid: Guid,
}

impl crate::Message for SMSG_BATTLEGROUND_PLAYER_JOINED {
    const OPCODE: u32 = 0x02ec;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // player_guid: Guid
        w.write_all(&self.player_guid.guid().to_le_bytes())?;

        Ok(())
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
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_BATTLEGROUND_PLAYER_JOINED {}

