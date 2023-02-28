use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/smsg_battleground_player_left.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/smsg_battleground_player_left.wowm#L3):
/// ```text
/// smsg SMSG_BATTLEGROUND_PLAYER_LEFT = 0x02ED {
///     Guid guid;
/// }
/// ```
pub struct SMSG_BATTLEGROUND_PLAYER_LEFT {
    pub guid: Guid,
}

impl crate::Message for SMSG_BATTLEGROUND_PLAYER_LEFT {
    const OPCODE: u32 = 0x02ed;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02ED, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            guid,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_BATTLEGROUND_PLAYER_LEFT {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_BATTLEGROUND_PLAYER_LEFT {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_BATTLEGROUND_PLAYER_LEFT {}

