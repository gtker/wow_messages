use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_propose_level_grant.wowm:7`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_propose_level_grant.wowm#L7):
/// ```text
/// smsg SMSG_PROPOSE_LEVEL_GRANT = 0x041F {
///     PackedGuid player;
/// }
/// ```
pub struct SMSG_PROPOSE_LEVEL_GRANT {
    pub player: Guid,
}

impl crate::Message for SMSG_PROPOSE_LEVEL_GRANT {
    const OPCODE: u32 = 0x041f;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // player: PackedGuid
        self.player.write_packed_guid_into_vec(&mut w)?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(2..=9).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x041F, size: body_size as u32 });
        }

        // player: PackedGuid
        let player = Guid::read_packed(r)?;

        Ok(Self {
            player,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_PROPOSE_LEVEL_GRANT {}

impl SMSG_PROPOSE_LEVEL_GRANT {
    pub(crate) fn size(&self) -> usize {
        self.player.size() // player: Guid
    }
}

