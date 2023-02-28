use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/smsg_battleground_player_joined.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/smsg_battleground_player_joined.wowm#L3):
/// ```text
/// smsg SMSG_BATTLEGROUND_PLAYER_JOINED = 0x02EC {
///     Guid player;
/// }
/// ```
pub struct SMSG_BATTLEGROUND_PLAYER_JOINED {
    pub player: Guid,
}

impl crate::Message for SMSG_BATTLEGROUND_PLAYER_JOINED {
    const OPCODE: u32 = 0x02ec;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // player: Guid
        w.write_all(&self.player.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02EC, size: body_size as u32 });
        }

        // player: Guid
        let player = Guid::read(r)?;

        Ok(Self {
            player,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_BATTLEGROUND_PLAYER_JOINED {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_BATTLEGROUND_PLAYER_JOINED {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_BATTLEGROUND_PLAYER_JOINED {}

