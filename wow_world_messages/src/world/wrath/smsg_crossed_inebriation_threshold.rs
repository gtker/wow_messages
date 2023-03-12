use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_crossed_inebriation_threshold.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_crossed_inebriation_threshold.wowm#L9):
/// ```text
/// smsg SMSG_CROSSED_INEBRIATION_THRESHOLD = 0x03C1 {
///     Guid player;
///     u32 state;
///     u32 item;
/// }
/// ```
pub struct SMSG_CROSSED_INEBRIATION_THRESHOLD {
    pub player: Guid,
    pub state: u32,
    pub item: u32,
}

impl crate::Message for SMSG_CROSSED_INEBRIATION_THRESHOLD {
    const OPCODE: u32 = 0x03c1;

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // player: Guid
        w.write_all(&self.player.guid().to_le_bytes())?;

        // state: u32
        w.write_all(&self.state.to_le_bytes())?;

        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03C1, size: body_size as u32 });
        }

        // player: Guid
        let player = Guid::read(&mut r)?;

        // state: u32
        let state = crate::util::read_u32_le(&mut r)?;

        // item: u32
        let item = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            player,
            state,
            item,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CROSSED_INEBRIATION_THRESHOLD {}

