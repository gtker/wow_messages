use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/msg_battleground_player_positions.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/msg_battleground_player_positions.wowm#L1):
/// ```text
/// cmsg MSG_BATTLEGROUND_PLAYER_POSITIONS_Client = 0x02E9 {
/// }
/// ```
pub struct MSG_BATTLEGROUND_PLAYER_POSITIONS_Client {
}

impl crate::private::Sealed for MSG_BATTLEGROUND_PLAYER_POSITIONS_Client {}
impl crate::Message for MSG_BATTLEGROUND_PLAYER_POSITIONS_Client {
    const OPCODE: u32 = 0x02e9;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02E9, size: body_size });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for MSG_BATTLEGROUND_PLAYER_POSITIONS_Client {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for MSG_BATTLEGROUND_PLAYER_POSITIONS_Client {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for MSG_BATTLEGROUND_PLAYER_POSITIONS_Client {}

