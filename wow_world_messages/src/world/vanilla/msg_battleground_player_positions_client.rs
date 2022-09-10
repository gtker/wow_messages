use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/msg_battleground_player_positions_client.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/msg_battleground_player_positions_client.wowm#L3):
/// ```text
/// cmsg MSG_BATTLEGROUND_PLAYER_POSITIONS_Client = 0x02E9 {
/// }
/// ```
pub struct MSG_BATTLEGROUND_PLAYER_POSITIONS_Client {
}

impl crate::Message for MSG_BATTLEGROUND_PLAYER_POSITIONS_Client {
    const OPCODE: u32 = 0x02e9;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        Ok(Self {
        })
    }

}
#[cfg(any(feature = "vanilla", feature = "tbc"))]
impl crate::helper::shared::vanilla_tbc::ClientMessage for MSG_BATTLEGROUND_PLAYER_POSITIONS_Client {}

