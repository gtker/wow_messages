use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/achievement/cmsg_query_inspect_achievements.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/achievement/cmsg_query_inspect_achievements.wowm#L1):
/// ```text
/// cmsg CMSG_QUERY_INSPECT_ACHIEVEMENTS = 0x046B {
///     PackedGuid player;
/// }
/// ```
pub struct CMSG_QUERY_INSPECT_ACHIEVEMENTS {
    pub player: Guid,
}

impl crate::Message for CMSG_QUERY_INSPECT_ACHIEVEMENTS {
    const OPCODE: u32 = 0x046b;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // player: PackedGuid
        self.player.write_packed_guid_into_vec(w);

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(2..=9).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x046B, size: body_size as u32 });
        }

        // player: PackedGuid
        let player = Guid::read_packed(r)?;

        Ok(Self {
            player,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_QUERY_INSPECT_ACHIEVEMENTS {}

impl CMSG_QUERY_INSPECT_ACHIEVEMENTS {
    pub(crate) fn size(&self) -> usize {
        self.player.size() // player: Guid
    }
}

