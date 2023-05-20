use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_propose_level_grant.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_propose_level_grant.wowm#L1):
/// ```text
/// smsg SMSG_PROPOSE_LEVEL_GRANT = 0x041E {
///     PackedGuid player;
/// }
/// ```
pub struct SMSG_PROPOSE_LEVEL_GRANT {
    pub player: Guid,
}

impl crate::private::Sealed for SMSG_PROPOSE_LEVEL_GRANT {}
impl crate::Message for SMSG_PROPOSE_LEVEL_GRANT {
    const OPCODE: u32 = 0x041e;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // player: PackedGuid
        self.player.write_packed_guid_into_vec(&mut w)?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(2..=9).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x041E, size: body_size });
        }

        // player: PackedGuid
        let player = Guid::read_packed(&mut r)?;

        Ok(Self {
            player,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_PROPOSE_LEVEL_GRANT {}

impl SMSG_PROPOSE_LEVEL_GRANT {
    pub(crate) const fn size(&self) -> usize {
        self.player.size() // player: PackedGuid
    }
}

