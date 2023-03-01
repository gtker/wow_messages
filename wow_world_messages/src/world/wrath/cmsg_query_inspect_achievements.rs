use crate:: {
    Guid,
};
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
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

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // player: PackedGuid
        self.player.write_packed_guid_into_vec(&mut w)?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(2..=9).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x046B, size: body_size as u32 });
        }

        // player: PackedGuid
        let player = Guid::read_packed(&mut r)?;

        Ok(Self {
            player,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_QUERY_INSPECT_ACHIEVEMENTS {}

impl CMSG_QUERY_INSPECT_ACHIEVEMENTS {
    pub(crate) fn size(&self) -> usize {
        self.player.size() // player: PackedGuid
    }
}

